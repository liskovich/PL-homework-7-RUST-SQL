use std::panic;

use crate::{
    constants::{
        GROUND_PLATFORM_COST, GROUND_PLATFORM_PROFITABILITY, GROUND_PLATFORM_UPGRADE_COST,
        PUMP_PLATFORM_COST, PUMP_PLATFORM_PROFITABILITY, PUMP_PLATFORM_UPGRADE_COST,
        RIG_PLATFORM_COST, RIG_PLATFORM_PROFITABILITY, RIG_PLATFORM_UPGRADE_COST,
    },
    model::{CreateMoneyTransactionModel, CreatePlatformModel, PlatformType, UpdatePlatformModel},
    repo::{generic::Repo, platform_repo::OilPlatformError},
    schema::{
        CreatePlatformSchema, CreatePlatformUpgradeSchema, GenericResponse, PlatformListResponse,
        SinglePlatformResponse,
    },
    AppRepositories,
};
use rocket::{get, http::Status, post, response::status::Custom, serde::json::Json, State};
use uuid::Uuid;

#[get("/health")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: "service alive".to_string(),
    };
    Ok(Json(response_json))
}

#[get("/platforms")]
pub async fn platforms_list_handler(
    data: &State<AppRepositories>,
) -> Result<Json<PlatformListResponse>, Status> {
    let oil_platform_repo = &data.platform_repo;

    let platforms = oil_platform_repo
        .get_all()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let json_response = PlatformListResponse {
        status: "success".to_string(),
        results: platforms.len(),
        platforms,
    };
    Ok(Json(json_response))
}

#[post("/platforms", data = "<body>")]
pub async fn create_todo_handler(
    body: Json<CreatePlatformSchema>,
    data: &State<AppRepositories>,
) -> Result<Json<SinglePlatformResponse>, Custom<Json<GenericResponse>>> {
    let create_request = body.into_inner();

    // validate platform type
    let platform_type_ref: &str = &create_request.platform_type;
    let result = panic::catch_unwind(|| platform_type_ref.into());

    let validated_platform_type = match result {
        Ok(validated) => validated,
        Err(e) => {
            println!("Panic occurred: {:?}", e);
            let error_response = Custom(
                Status::BadRequest,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "Invalid platform type specified".to_string(),
                }),
            );
            return Err(error_response);
        }
    };

    // validate amount of available money
    let finances_repo = &data.finances_repo;
    let balance = match finances_repo.get_available_balance().await {
        Ok(bal) => bal,
        Err(_) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "Failed to create platform".to_string(),
                }),
            );
            return Err(error_response);
        }
    };
    let cost = get_platform_cost(validated_platform_type);
    if cost > balance {
        let error_response = Custom(
            Status::BadRequest,
            Json(GenericResponse {
                status: "error".to_string(),
                message: "Not enough funds for purchase".to_string(),
            }),
        );
        return Err(error_response);
    }

    // save to database
    let platform_create = CreatePlatformModel {
        platform_type: validated_platform_type,
        profitability: get_platform_profitability(validated_platform_type),
    };
    let oil_platform_repo = &data.platform_repo;
    let created = match oil_platform_repo.create(platform_create).await {
        Ok(platform) => platform,
        Err(_) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "Failed to create platform".to_string(),
                }),
            );
            return Err(error_response);
        }
    };

    // create associated money transaction
    let platform_create_tx = CreateMoneyTransactionModel {
        item_id: Some(created.id),
        amount: cost,
        reduces_balance: true,
    };
    match finances_repo.create(platform_create_tx).await {
        Ok(_) => (),
        Err(e) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: format!("Failed to create platform: {}", e.to_string()),
                }),
            );
            return Err(error_response);
        }
    };

    let json_response = SinglePlatformResponse {
        status: "success".to_string(),
        data: created,
    };
    Ok(Json(json_response))
}

#[patch("/platforms/<id>", data = "<body>")]
pub async fn edit_platform_handler(
    id: String,
    body: Json<CreatePlatformUpgradeSchema>,
    data: &State<AppRepositories>,
) -> Result<Json<SinglePlatformResponse>, Custom<Json<GenericResponse>>> {
    // validate provided id
    let uuid = match Uuid::parse_str(&id) {
        Ok(res) => res,
        Err(_) => {
            let error_response = Custom(
                Status::BadRequest,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "Invalid ID provided".to_string(),
                }),
            );
            return Err(error_response);
        }
    };

    // check if exists
    let oil_platform_repo = &data.platform_repo;
    let retrieved = match oil_platform_repo.get_by_id(uuid).await {
        Ok(platform) => platform,
        Err(e) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                }),
            );
            return Err(error_response);
        }
    };

    // validate amount of available money
    let finances_repo = &data.finances_repo;
    let balance = match finances_repo.get_available_balance().await {
        Ok(bal) => bal,
        Err(_) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "Failed to upgrade platform".to_string(),
                }),
            );
            return Err(error_response);
        }
    };
    let cost = get_platform_upgrade_cost(retrieved.platform_type);
    if cost > balance {
        let error_response = Custom(
            Status::BadRequest,
            Json(GenericResponse {
                status: "error".to_string(),
                message: "Not enough funds for purchase".to_string(),
            }),
        );
        return Err(error_response);
    }

    // update and save
    let platform_update = UpdatePlatformModel {
        profitability_addition: get_platform_profitability(retrieved.platform_type),
    };
    let updated = match oil_platform_repo.update(uuid, platform_update).await {
        Ok(platform) => platform,
        Err(OilPlatformError::MaxLevelReached) => {
            let error_response = Custom(
                Status::BadRequest,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "You have already upgraded the platform to the maximum".to_string(),
                }),
            );
            return Err(error_response);
        }
        Err(e) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                }),
            );
            return Err(error_response);
        }
    };

    // create associated money transaction
    let platform_update_tx = CreateMoneyTransactionModel {
        item_id: Some(updated.id),
        amount: cost,
        reduces_balance: true,
    };
    match finances_repo.create(platform_update_tx).await {
        Ok(_) => (),
        Err(e) => {
            let error_response = Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: format!("Failed to create platform: {}", e.to_string()),
                }),
            );
            return Err(error_response);
        }
    };

    let json_response = SinglePlatformResponse {
        status: "success".to_string(),
        data: updated,
    };
    Ok(Json(json_response))
}

// oil platform helpers
fn get_platform_cost(platform: PlatformType) -> i64 {
    match platform {
        PlatformType::Rig => RIG_PLATFORM_COST,
        PlatformType::Ground => GROUND_PLATFORM_COST,
        PlatformType::Pump => PUMP_PLATFORM_COST,
    }
}

fn get_platform_upgrade_cost(platform: PlatformType) -> i64 {
    match platform {
        PlatformType::Rig => RIG_PLATFORM_UPGRADE_COST,
        PlatformType::Ground => GROUND_PLATFORM_UPGRADE_COST,
        PlatformType::Pump => PUMP_PLATFORM_UPGRADE_COST,
    }
}

fn get_platform_profitability(platform: PlatformType) -> i64 {
    match platform {
        PlatformType::Rig => RIG_PLATFORM_PROFITABILITY,
        PlatformType::Ground => GROUND_PLATFORM_PROFITABILITY,
        PlatformType::Pump => PUMP_PLATFORM_PROFITABILITY,
    }
}
