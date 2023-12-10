mod constants;
mod model;
mod schema;

use dotenv::dotenv;

use sqlx::postgres::PgPoolOptions;

mod repo;
use repo::{generic::Repo, platform_repo::OilPlaftormRepo};

#[macro_use]
extern crate rocket;
use rocket::http::Method;
use rocket::tokio::time::{sleep, Duration};
use rocket_cors::{AllowedOrigins, CorsOptions};

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

    // initialize repos
    let oil_platform_repo = OilPlaftormRepo::new(pool.clone());

    let all_platforms = oil_platform_repo.get_all().await;
    println!("{:?}", all_platforms);

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
        .mount("/hello", routes![index])
        .attach(cors.to_cors().unwrap())
        .launch()
        .await
        .expect("Failed to start server");

    Ok(())
}

#[get("/")]
async fn index() -> &'static str {
    sleep(Duration::from_secs(3)).await;
    "Hello, world!"
}
