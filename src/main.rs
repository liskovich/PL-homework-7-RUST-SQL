mod constants;
mod game_loop;
mod handler;
mod helper;
mod model;
mod repo;
mod schema;

use dotenv::dotenv;

use repo::{
    beer_repo::BeerRepo, generic::Repo, platform_repo::OilPlaftormRepo,
    transactions_repo::TransactionsRepo,
};

use helper::seed_game_entities;

use sqlx::{postgres::PgPoolOptions, PgPool};

#[macro_use]
extern crate rocket;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

use handler::{
    beer_handler::{beers_list_handler, purchase_beer_handler},
    platform_handler::{
        create_platform_handler, edit_platform_handler, health_checker_handler,
        platforms_list_handler,
    },
};

use game_loop::echo_stream;

struct AppRepositories {
    platform_repo: OilPlaftormRepo,
    beer_repo: BeerRepo,
    finances_repo: TransactionsRepo,
}

async fn initialize_repositories(pool: &PgPool) -> AppRepositories {
    let platform_repo = OilPlaftormRepo::new(pool.clone());
    let beer_repo = BeerRepo::new(pool.clone());
    let finances_repo = TransactionsRepo::new(pool.clone());
    AppRepositories {
        platform_repo,
        beer_repo,
        finances_repo,
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    // setup database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(()) => {
            println!("Database migrations successful!");
        }
        Err(err) => {
            println!("Failed to migrate the database: {:?}", err);
            std::process::exit(1);
        }
    }

    // setup services and run server
    let repositories = initialize_repositories(&pool).await;
    seed_game_entities(&pool).await;

    // TODO: adjust for production
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let _rocket = rocket::build()
        .manage(repositories)
        .mount(
            "/api",
            routes![
                health_checker_handler,
                platforms_list_handler,
                create_platform_handler,
                edit_platform_handler,
                beers_list_handler,
                purchase_beer_handler,
                echo_stream,
            ],
        )
        .attach(cors.to_cors().unwrap())
        .launch()
        .await
        .expect("Failed to start server");

    Ok(())
}
