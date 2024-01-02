use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

use crate::model::BeerModel;

#[derive(Debug)]
pub enum BeerError {
    NotFound,
    AlreadyPurchased,
    OtherError,
}

impl std::fmt::Display for BeerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BeerError::NotFound => write!(f, "Beer not found"),
            BeerError::AlreadyPurchased => {
                write!(f, "Beer already purchased")
            }
            BeerError::OtherError => write!(f, "Other database-related error"),
        }
    }
}

impl std::error::Error for BeerError {}

#[async_trait]
pub trait BeerRepoTrait: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<BeerModel, BeerError>;
    async fn get_all(&self) -> Result<Vec<BeerModel>, BeerError>;
    async fn purchase(&self, id: Uuid) -> Result<BeerModel, BeerError>;
}

pub struct BeerRepo {
    pool: PgPool,
}

impl BeerRepo {
    pub fn new(pool: PgPool) -> Self {
        BeerRepo { pool }
    }
}

#[async_trait]
impl BeerRepoTrait for BeerRepo {
    async fn get_by_id(&self, id: Uuid) -> Result<BeerModel, BeerError> {
        let query_result = match sqlx::query_as!(BeerModel, "SELECT * FROM beers WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(beer) => beer,
            Err(SqlxError::RowNotFound) => return Err(BeerError::NotFound),
            Err(_) => return Err(BeerError::OtherError),
        };

        Ok(query_result)
    }

    async fn get_all(&self) -> Result<Vec<BeerModel>, BeerError> {
        let query_result = match sqlx::query_as!(BeerModel, "SELECT * FROM beers ORDER BY cost ASC")
            .fetch_all(&self.pool)
            .await
        {
            Ok(beers) => beers,
            Err(SqlxError::RowNotFound) => return Err(BeerError::NotFound),
            Err(_) => return Err(BeerError::OtherError),
        };

        Ok(query_result)
    }

    async fn purchase(&self, id: Uuid) -> Result<BeerModel, BeerError> {
        let query_result = match sqlx::query_as!(BeerModel, "SELECT * FROM beers WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(beer) => beer,
            Err(SqlxError::RowNotFound) => return Err(BeerError::NotFound),
            Err(_) => return Err(BeerError::OtherError),
        };

        if query_result.purchased == Some(true) {
            return Err(BeerError::AlreadyPurchased);
        }

        let query_result = match sqlx::query_as!(
            BeerModel,
            "UPDATE beers SET purchased = $1 WHERE id = $2 RETURNING *",
            true,
            id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(beer_purchased) => beer_purchased,
            Err(SqlxError::RowNotFound) => return Err(BeerError::NotFound),
            Err(_) => return Err(BeerError::OtherError),
        };

        Ok(query_result)
    }
}
