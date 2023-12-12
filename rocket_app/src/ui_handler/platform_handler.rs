use std::panic;

use rocket::form::{Form, Strict};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::{
    helper::{get_platform_cost, get_platform_profitability, get_platform_upgrade_cost},
    schema::CreatePlatformSchema,
    AppRepositories,
};
use shared_db::{
    model::{CreateMoneyTransactionModel, CreatePlatformModel, UpdatePlatformModel},
    repo::{generic::Repo, platform_repo::OilPlatformError},
};

use rocket::{get, post, State};
use uuid::Uuid;

#[get("/platforms/create")]
pub async fn get_create_platform_ui_handler(data: &State<AppRepositories>) -> Template {
    // get money balance info
    let finances_repo = &data.finances_repo;
    let balance = match finances_repo.get_available_balance().await {
        Ok(bal) => bal,
        Err(_) => {
            return Template::render(
                "error/500",
                context! {
                    error: "Failed to get available balance".to_string(),
                },
            );
        }
    };

    Template::render(
        "create_platform",
        context! {
            balance: balance,
        },
    )
}

#[post("/platforms/create", data = "<body>")]
pub async fn create_platform_ui_handler(
    body: Form<Strict<CreatePlatformSchema>>,
    data: &State<AppRepositories>,
) -> Result<Redirect, Template> {
    let create_request = body.into_inner().into_inner();

    // validate platform type
    let platform_type_ref: &str = &create_request.platform_type;
    let result = panic::catch_unwind(|| platform_type_ref.into());

    let validated_platform_type = match result {
        Ok(validated) => validated,
        Err(e) => {
            println!("Panic occurred: {:?}", e);
            let error_response = Template::render(
                "error/500",
                context! {
                    error: "Invalid platform type specified".to_string(),
                },
            );
            return Err(error_response);
        }
    };

    // validate amount of available money
    let finances_repo = &data.finances_repo;
    let balance = match finances_repo.get_available_balance().await {
        Ok(bal) => bal,
        Err(_) => {
            let error_response = Template::render(
                "error/500",
                context! {
                    error: "Failed to create platform".to_string(),
                },
            );
            return Err(error_response);
        }
    };
    let cost = get_platform_cost(validated_platform_type);
    if cost > balance {
        let error_response = Template::render(
            "error/400",
            context! {
                error: "Not enough funds for purchase".to_string(),
            },
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
        Err(e) => {
            let error_response = Template::render(
                "error/500",
                context! {
                    error: format!("Failed to create platform: {}", e.to_string()),
                },
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
            let error_response = Template::render(
                "error/500",
                context! {
                    error: format!("Failed to create platform: {}", e.to_string()),
                },
            );
            return Err(error_response);
        }
    };

    Ok(Redirect::to(uri!("/")))
}

#[post("/platforms/edit/<id>")]
pub async fn upgrade_platform_ui_handler(
    id: String,
    data: &State<AppRepositories>,
) -> Result<Redirect, Template> {
    // validate provided id
    let uuid = match Uuid::parse_str(&id) {
        Ok(res) => res,
        Err(_) => {
            let error_response = Template::render(
                "error/400",
                context! {
                    error: "Invalid ID provided".to_string(),
                },
            );
            return Err(error_response);
        }
    };

    // check if exists
    let oil_platform_repo = &data.platform_repo;
    let retrieved = match oil_platform_repo.get_by_id(uuid).await {
        Ok(platform) => platform,
        Err(e) => {
            let error_response = Template::render(
                "error/500",
                context! {
                    error: e.to_string(),
                },
            );
            return Err(error_response);
        }
    };

    // validate amount of available money
    let finances_repo = &data.finances_repo;
    let balance = match finances_repo.get_available_balance().await {
        Ok(bal) => bal,
        Err(_) => {
            let error_response = Template::render(
                "error/500",
                context! {
                    error: "Failed to upgrade platform".to_string(),
                },
            );
            return Err(error_response);
        }
    };
    let cost = get_platform_upgrade_cost(retrieved.platform_type);
    if cost > balance {
        let error_response = Template::render(
            "error/400",
            context! {
                error: "Not enough funds for purchase".to_string(),
            },
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
            let error_response = Template::render(
                "error/400",
                context! {
                    error: "You have already upgraded the platform to the maximum".to_string(),
                },
            );
            return Err(error_response);
        }
        Err(e) => {
            let error_response = Template::render(
                "error/500",
                context! {
                    error: e.to_string(),
                },
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
            let error_response = Template::render(
                "error/500",
                context! {
                    error: format!("Failed to create platform: {}", e.to_string()),
                },
            );
            return Err(error_response);
        }
    };

    Ok(Redirect::to("/"))
}
