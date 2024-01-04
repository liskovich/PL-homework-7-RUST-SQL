use rocket::FromForm;
use serde::{Deserialize, Serialize};
use shared_db::{
    model::{BeerModel, OilPlatformModel},
    repo::{
        beer_repo::BeerRepoTrait, platform_repo::OilPlaftormRepoTrait,
        transactions_repo::TransactionsRepoTrait,
    },
};

// requests
#[derive(Deserialize, Debug, Clone)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct CreatePlatformSchema {
    pub platform_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePlatformUpgradeSchema;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMoneyTransactionSchema {
    pub item_id: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurchaseBeerSchema;

// responses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SinglePlatformResponse {
    pub status: String,
    pub data: OilPlatformModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformListResponse {
    pub status: String,
    pub results: usize,
    pub platforms: Vec<OilPlatformModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleBeerResponse {
    pub status: String,
    pub data: BeerModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeerListResponse {
    pub status: String,
    pub results: usize,
    pub beers: Vec<BeerModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub struct AppRepositories {
    pub platform_repo: Box<dyn OilPlaftormRepoTrait>,
    pub beer_repo: Box<dyn BeerRepoTrait>,
    pub finances_repo: Box<dyn TransactionsRepoTrait>,
}
