use crate::{model::CreateMoneyTransactionModel, repo::beer_repo::BeerError, AppRepositories};
use rocket::{get, response::Redirect, State};
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

#[get("/beers")]
pub async fn beers_handler(data: &State<AppRepositories>) -> Template {
    let beers_repo = &data.beer_repo;

    match beers_repo.get_all().await {
        Ok(res) => Template::render(
            "tera/beers",
            context! {
                items: res,
            },
        ),
        Err(e) => Template::render(
            "tera/error/500",
            context! {
                error: e.to_string(),
            },
        ),
    }
}

#[get("/beers/create")]
pub async fn get_purchase_beer_ui_handler() -> Template {
    Template::render("tera/create_beer", context! {})
}

#[patch("/beers/<id>")]
pub async fn purchase_beer_ui_handler(
    id: String,
    data: &State<AppRepositories>,
) -> Result<Redirect, Template> {
    // validate provided id
    let uuid = match Uuid::parse_str(&id) {
        Ok(res) => res,
        Err(_) => {
            let error_response = Template::render(
                "tera/error/400",
                context! {
                    error: "Invalid ID provided".to_string(),
                },
            );
            return Err(error_response);
        }
    };

    // check if exists
    let beer_repo = &data.beer_repo;
    let retrieved = match beer_repo.get_by_id(uuid).await {
        Ok(beer) => beer,
        Err(e) => {
            let error_response = Template::render(
                "tera/error/500",
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
                "tera/error/500",
                context! {
                    error: "Failed to upgrade beer".to_string(),
                },
            );
            return Err(error_response);
        }
    };
    if retrieved.cost > balance {
        let error_response = Template::render(
            "tera/error/400",
            context! {
                error: "Not enough funds for purchase".to_string(),
            },
        );
        return Err(error_response);
    }

    // update and save
    let updated = match beer_repo.purchase(uuid).await {
        Ok(platform) => platform,
        Err(BeerError::AlreadyPurchased) => {
            let error_response = Template::render(
                "tera/error/400",
                context! {
                    error: "You have already purchased this beer".to_string(),
                },
            );
            return Err(error_response);
        }
        Err(e) => {
            let error_response = Template::render(
                "tera/error/500",
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
        amount: updated.cost,
        reduces_balance: true,
    };
    match finances_repo.create(platform_update_tx).await {
        Ok(_) => (),
        Err(e) => {
            let error_response = Template::render(
                "tera/error/500",
                context! {
                    error: format!("Failed to create platform: {}", e.to_string()),
                },
            );
            return Err(error_response);
        }
    };

    Ok(Redirect::to("/beers"))
}
