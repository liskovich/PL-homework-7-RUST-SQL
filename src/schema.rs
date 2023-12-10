use crate::model::OilPlatformModel;
use serde::{Deserialize, Serialize};

// requests
#[derive(Deserialize, Debug, Clone)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePlatformSchema {
    pub platform_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePlatformSchema {
    pub new_platform_level: i16,
    pub new_profitability: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePlatformUpgradeSchema {
    pub platform_id: String,
    pub new_platform_level: i16,
    pub profitability_addition: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMoneyTransactionSchema {
    pub item_id: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateBeerSchema {
    pub title: String,
    pub description: String,
    pub cost: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateBeerSchema {
    pub cost: i64,
}

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
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
