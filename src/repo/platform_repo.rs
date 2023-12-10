use async_trait::async_trait;
use sqlx::Error as SqlxError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::constants::MAX_PLATFORM_LEVEL;
use crate::model::CreatePlatformModel;
use crate::model::OilPlatformModel;
use crate::schema::UpdatePlatformSchema;

use super::generic::Repo;

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

pub struct OilPlaftormRepo {
    pool: PgPool,
}

#[async_trait]
impl Repo<OilPlatformModel, CreatePlatformModel, UpdatePlatformSchema> for OilPlaftormRepo {
    type Error = OilPlatformError;
    type Pool = PgPool;

    fn new(pool: Self::Pool) -> Self {
        OilPlaftormRepo { pool }
    }

    fn get_pool(&self) -> Self::Pool {
        self.pool.clone()
    }

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
        let query_result = match sqlx::query_as!(OilPlatformModel, "SELECT * FROM oil_platforms")
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
        new_item: UpdatePlatformSchema,
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
            new_item.new_platform_level.to_owned(),
            new_item.new_profitability.to_owned(),
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
