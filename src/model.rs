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

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize, sqlx::Type)]
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
pub struct OilPlatformUpgradeModel {
    pub id: Uuid,
    pub platform_id: Uuid,
    pub new_platform_level: i16,
    pub profitability_addition: i64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct MoneyTransactionModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub amount: i64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct BeerModel {
    pub id: Uuid,
    pub title: Uuid,
    pub description: String,
    pub cost: i64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}
