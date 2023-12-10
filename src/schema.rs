use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlatformSchema {
    pub platform_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlatformSchema {
    pub new_platform_level: i16,
    pub new_profitability: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlatformUpgradeSchema {
    pub platform_id: String,
    pub new_platform_level: i16,
    pub profitability_addition: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMoneyTransactionSchema {
    pub item_id: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBeerSchema {
    pub title: String,
    pub description: String,
    pub cost: i64,
}
