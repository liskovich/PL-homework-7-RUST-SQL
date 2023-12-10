use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::BeerModel;
use crate::schema::CreateBeerSchema;
use crate::schema::UpdateBeerSchema;

use super::generic::Repo;

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

#[async_trait]
impl Repo<BeerModel, CreateBeerSchema, UpdateBeerSchema> for BeerRepo {
    type Error = BeerError;
    type Pool = PgPool;

    fn new(pool: Self::Pool) -> Self {
        BeerRepo { pool }
    }

    fn get_pool(&self) -> Self::Pool {
        self.pool.clone()
    }

    async fn get_by_id(&self, id: Uuid) -> Result<BeerModel, BeerError> {
        todo!()
    }

    async fn get_all(&self) -> Result<Vec<BeerModel>, BeerError> {
        todo!()
    }

    async fn create(&self, item: CreateBeerSchema) -> Result<BeerModel, BeerError> {
        todo!()
    }

    async fn update(&self, id: Uuid, new_item: UpdateBeerSchema) -> Result<BeerModel, BeerError> {
        todo!()
    }
}
