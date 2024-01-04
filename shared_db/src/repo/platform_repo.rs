use async_trait::async_trait;
use sqlx::Error as SqlxError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::constants::MAX_PLATFORM_LEVEL;
use crate::model::{CreatePlatformModel, OilPlatformModel, UpdatePlatformModel};

#[derive(Debug)]
pub enum OilPlatformError {
    NotFound,
    MaxLevelReached,
    OtherError,
}

impl std::fmt::Display for OilPlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OilPlatformError::NotFound => write!(f, "Platform not found"),
            OilPlatformError::MaxLevelReached => {
                write!(f, "Maximum upgrade level of platform reached")
            }
            OilPlatformError::OtherError => write!(f, "Other database-related error"),
        }
    }
}

impl std::error::Error for OilPlatformError {}

#[async_trait]
pub trait OilPlaftormRepoTrait: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<OilPlatformModel, OilPlatformError>;
    async fn get_all(&self) -> Result<Vec<OilPlatformModel>, OilPlatformError>;
    async fn create(&self, item: CreatePlatformModel)
        -> Result<OilPlatformModel, OilPlatformError>;
    async fn update(
        &self,
        id: Uuid,
        new_item: UpdatePlatformModel,
    ) -> Result<OilPlatformModel, OilPlatformError>;
}

pub struct OilPlaftormRepo {
    pool: PgPool,
}

impl OilPlaftormRepo {
    pub fn new(pool: PgPool) -> Self {
        OilPlaftormRepo { pool }
    }
}

#[async_trait]
impl OilPlaftormRepoTrait for OilPlaftormRepo {
    async fn get_by_id(&self, id: Uuid) -> Result<OilPlatformModel, OilPlatformError> {
        let query_result = match sqlx::query_as!(
            OilPlatformModel,
            "SELECT * FROM oil_platforms WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(platform) => platform,
            Err(SqlxError::RowNotFound) => return Err(OilPlatformError::NotFound),
            Err(_) => return Err(OilPlatformError::OtherError),
        };

        Ok(query_result)
    }

    async fn get_all(&self) -> Result<Vec<OilPlatformModel>, OilPlatformError> {
        let query_result = match sqlx::query_as!(
            OilPlatformModel,
            "SELECT * FROM oil_platforms ORDER BY created_at ASC"
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(platforms) => platforms,
            Err(SqlxError::RowNotFound) => return Err(OilPlatformError::NotFound),
            Err(_) => return Err(OilPlatformError::OtherError),
        };

        Ok(query_result)
    }

    async fn create(
        &self,
        item: CreatePlatformModel,
    ) -> Result<OilPlatformModel, OilPlatformError> {
        let query_result = match sqlx::query_as!(
            OilPlatformModel,
            "INSERT INTO oil_platforms (platform_type, profitability) VALUES ($1, $2) RETURNING *",
            item.platform_type.to_string(),
            item.profitability,
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(platform) => platform,
            Err(_) => return Err(OilPlatformError::OtherError),
        };

        Ok(query_result)
    }

    async fn update(
        &self,
        id: Uuid,
        new_item: UpdatePlatformModel,
    ) -> Result<OilPlatformModel, OilPlatformError> {
        let query_result = match sqlx::query_as!(
            OilPlatformModel,
            "SELECT * FROM oil_platforms WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(platform) => platform,
            Err(SqlxError::RowNotFound) => return Err(OilPlatformError::NotFound),
            Err(_) => return Err(OilPlatformError::OtherError),
        };

        if query_result.platform_level == MAX_PLATFORM_LEVEL {
            return Err(OilPlatformError::MaxLevelReached);
        }

        let query_result = match sqlx::query_as!(
            OilPlatformModel,
            "UPDATE oil_platforms SET platform_level = $1, profitability = $2, updated_at = $3 WHERE id = $4 RETURNING *",
            query_result.platform_level + 1,
            query_result.profitability + new_item.profitability_addition,
            chrono::Utc::now().timestamp(),
            id
        )
        .fetch_one(&self.pool)
        .await {
            Ok(platform_upgraded) => platform_upgraded,
            Err(SqlxError::RowNotFound) => return Err(OilPlatformError::NotFound),
            Err(_) => return Err(OilPlatformError::OtherError),
        };

        Ok(query_result)
    }
}
