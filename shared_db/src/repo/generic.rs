use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repo<ReturnType, CreateType, UpdateType> {
    type Error;
    type Pool: Clone;

    fn new(pool: Self::Pool) -> Self;
    fn get_pool(&self) -> Self::Pool;

    async fn get_by_id(&self, id: Uuid) -> Result<ReturnType, Self::Error>;
    async fn get_all(&self) -> Result<Vec<ReturnType>, Self::Error>;
    async fn create(&self, item: CreateType) -> Result<ReturnType, Self::Error>;
    async fn update(&self, id: Uuid, new_item: UpdateType) -> Result<ReturnType, Self::Error>;
}
