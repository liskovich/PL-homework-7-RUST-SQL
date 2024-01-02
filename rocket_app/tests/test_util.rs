use async_trait::async_trait;
use mockall::mock;
use rocket::{http::Method, local::asynchronous::Client, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_dyn_templates::Template;
use shared_db::{
    model::{
        BeerModel, CreateMoneyTransactionModel, CreatePlatformModel, MoneyTransactionModel,
        OilPlatformModel, UpdatePlatformModel,
    },
    repo::{
        beer_repo::{BeerError, BeerRepoTrait},
        platform_repo::{OilPlaftormRepoTrait, OilPlatformError},
        transactions_repo::{MoneyTransactionError, TransactionsRepoTrait},
    },
};
use uuid::Uuid;
use dotenv::dotenv;

use rocket_app::{
    handler::{
        beer_handler::{beers_list_handler, purchase_beer_handler},
        platform_handler::{
            create_platform_handler, edit_platform_handler, health_checker_handler,
            platforms_list_handler,
        },
    },
    schema::AppRepositories,
};

use rocket_app::ui_handler::{
    beer_handler::purchase_beer_ui_handler,
    common_handler::{game_won_handler, index_handler},
    platform_handler::{
        create_platform_ui_handler, get_create_platform_ui_handler, upgrade_platform_ui_handler,
    },
};

mock! {
    pub BeerRepo {}

    #[async_trait]
    impl BeerRepoTrait for BeerRepo {
        async fn get_by_id(&self, id: Uuid) -> Result<BeerModel, BeerError>;
        async fn get_all(&self) -> Result<Vec<BeerModel>, BeerError>;
        async fn purchase(&self, id: Uuid) -> Result<BeerModel, BeerError>;
    }
}

mock! {
    pub OilPlaftormRepo {}

    #[async_trait]
    impl OilPlaftormRepoTrait for OilPlaftormRepo {
        async fn get_by_id(&self, id: Uuid) -> Result<OilPlatformModel, OilPlatformError>;
        async fn get_all(&self) -> Result<Vec<OilPlatformModel>, OilPlatformError>;
        async fn create(&self, item: CreatePlatformModel) -> Result<OilPlatformModel, OilPlatformError>;
        async fn update(&self, id: Uuid, new_item: UpdatePlatformModel) -> Result<OilPlatformModel, OilPlatformError>;
    }
}

mock! {
    pub TransactionsRepo {}

    #[async_trait]
    impl TransactionsRepoTrait for TransactionsRepo {
        async fn get_available_balance(&self) -> Result<i64, MoneyTransactionError>;
        async fn get_period_platform_earnings(&self) -> Result<i64, MoneyTransactionError>;
        async fn get_all(&self) -> Result<Vec<MoneyTransactionModel>, MoneyTransactionError>;
        async fn create(
            &self,
            item: CreateMoneyTransactionModel,
        ) -> Result<MoneyTransactionModel, MoneyTransactionError>;
    }
}

pub async fn aget_rocket_client(repos: AppRepositories) -> Client {
    dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let rocket = rocket::build()
        .manage(repos)
        .mount(
            "/api",
            routes![
                health_checker_handler,
                platforms_list_handler,
                create_platform_handler,
                edit_platform_handler,
                beers_list_handler,
                purchase_beer_handler,
            ],
        )
        .mount(
            "/",
            routes![
                index_handler,
                get_create_platform_ui_handler,
                create_platform_ui_handler,
                upgrade_platform_ui_handler,
                purchase_beer_ui_handler,
                game_won_handler,
            ],
        )
        .attach(cors.to_cors().unwrap())
        .attach(Template::fairing());

    let client = Client::tracked(rocket).await.expect("valid `Rocket`");
    client
}
