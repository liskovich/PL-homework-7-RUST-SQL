use crate::schema::{AppRepositories, BeerListResponse, GenericResponse, SingleBeerResponse};
use rocket::{get, http::Status, patch, response::status::Custom, serde::json::Json, State};
use shared_db::{model::CreateMoneyTransactionModel, repo::beer_repo::BeerError};
use uuid::Uuid;

/// Get a list of beers
///
/// Retrieves the list of all beers from the database.
/// Returns them in form of a JSON object.
#[get("/beers")]
pub async fn beers_list_handler(
    data: &State<AppRepositories>,
) -> Result<Json<BeerListResponse>, Status> {
    let beers_repo = &data.beer_repo;

    let beers = beers_repo
        .get_all()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let json_response = BeerListResponse {
        status: "success".to_string(),
        results: beers.len(),
        beers,
    };
    Ok(Json(json_response))
}

/// Purchase a beer
///
/// Given the identifier of a beer, marks it as purchased in the database.
/// Additionaly, creates a related money transaction to store purchase information in the database.
/// In case of success, returns the purchased beer in form of a JSON object. Otherwise, returns error message in form of a JSON object.
#[patch("/beers/<id>")]
pub async fn purchase_beer_handler(
    id: String,
    data: &State<AppRepositories>,
) -> Result<Json<SingleBeerResponse>, Custom<Json<GenericResponse>>> {
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
    let beer_repo = &data.beer_repo;
    let retrieved = match beer_repo.get_by_id(uuid).await {
        Ok(beer) => beer,
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
                    message: "Failed to upgrade beer".to_string(),
                }),
            );
            return Err(error_response);
        }
    };
    if retrieved.cost > balance {
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
    let updated = match beer_repo.purchase(uuid).await {
        Ok(platform) => platform,
        Err(BeerError::AlreadyPurchased) => {
            let error_response = Custom(
                Status::BadRequest,
                Json(GenericResponse {
                    status: "error".to_string(),
                    message: "You have already purchased this beer".to_string(),
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
        amount: updated.cost,
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

    let json_response = SingleBeerResponse {
        status: "success".to_string(),
        data: updated,
    };
    Ok(Json(json_response))
}
