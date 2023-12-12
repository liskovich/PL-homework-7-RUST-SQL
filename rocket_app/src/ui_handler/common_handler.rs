use crate::AppRepositories;
use rocket::{get, State};
use rocket_dyn_templates::{context, Template};
use shared_db::repo::generic::Repo;

#[get("/")]
pub async fn index_handler(data: &State<AppRepositories>) -> Template {
    let beers_repo = &data.beer_repo;
    let oil_platform_repo = &data.platform_repo;

    let beers = match beers_repo.get_all().await {
        Ok(res) => res,
        Err(e) => {
            return Template::render(
                "error/500",
                context! {
                    error: e.to_string(),
                },
            );
        }
    };
    let platforms = match oil_platform_repo.get_all().await {
        Ok(res) => res,
        Err(e) => {
            return Template::render(
                "error/500",
                context! {
                    error: e.to_string(),
                },
            );
        }
    };

    Template::render(
        "index",
        context! {
            platforms: platforms,
            beers: beers,
        },
    )
}

#[get("/win")]
pub async fn game_won_handler(data: &State<AppRepositories>) -> Template {
    let platform_repo = &data.platform_repo;
    let finance_repo = &data.finances_repo;

    let platforms = match platform_repo.get_all().await {
        Ok(res) => res,
        Err(e) => {
            return Template::render(
                "error/500",
                context! {
                    error: e.to_string(),
                },
            );
        }
    };
    let txs = match finance_repo.get_all().await {
        Ok(res) => res,
        Err(e) => {
            return Template::render(
                "error/500",
                context! {
                    error: e.to_string(),
                },
            );
        }
    };

    // prepare game summary
    let total_earned = txs
        .iter()
        .filter(|tx| !tx.reduces_balance)
        .map(|tx| tx.amount)
        .sum::<i64>();
    let total_spent = txs
        .iter()
        .filter(|tx| tx.reduces_balance)
        .map(|tx| tx.amount)
        .sum::<i64>();

    Template::render(
        "game_over",
        context! {
            platforms: platforms,
            earned: total_earned,
            spent: total_spent,
        },
    )
}
