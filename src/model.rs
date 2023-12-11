use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct OilPlatformModel {
    pub id: Uuid,
    pub platform_type: PlatformType,
    pub platform_level: i16,
    pub profitability: i64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CreatePlatformModel {
    pub platform_type: PlatformType,
    pub profitability: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePlatformModel {
    pub profitability_addition: i64,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "platform_type", rename_all = "lowercase")]
pub enum PlatformType {
    Rig,
    Ground,
    Pump,
}

impl From<String> for PlatformType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Rig" => PlatformType::Rig,
            "Ground" => PlatformType::Ground,
            "Pump" => PlatformType::Pump,
            _ => panic!("Unknown platform type: {}", s),
        }
    }
}

impl From<&str> for PlatformType {
    fn from(s: &str) -> Self {
        PlatformType::from(String::from(s))
    }
}

impl ToString for PlatformType {
    fn to_string(&self) -> String {
        match self {
            PlatformType::Rig => String::from("Rig"),
            PlatformType::Ground => String::from("Ground"),
            PlatformType::Pump => String::from("Pump"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct MoneyTransactionModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub amount: i64,
    pub reduces_balance: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMoneyTransactionModel {
    pub item_id: Option<Uuid>,
    pub amount: i64,
    pub reduces_balance: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct BeerModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
    pub cost: i64,
    pub purchased: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateBeerModel {
    pub title: String,
    pub thumbnail: String,
    pub description: String,
    pub cost: i64,
}
