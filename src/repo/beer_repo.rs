use sqlx::PgPool;
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

pub struct BeerRepo {
    pool: PgPool,
}

impl BeerRepo {
    pub fn new(pool: PgPool) -> Self {
        BeerRepo { pool }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<BeerModel, BeerError> {
        todo!()
    }

    pub async fn get_all(&self) -> Result<Vec<BeerModel>, BeerError> {
        todo!()
    }

    pub async fn purchase(&self, id: Uuid) -> Result<BeerModel, BeerError> {
        todo!()
    }
}
