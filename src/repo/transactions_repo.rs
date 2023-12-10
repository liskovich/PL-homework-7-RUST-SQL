use async_trait::async_trait;
use sqlx::Error as SqlxError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::MoneyTransactionModel;
use crate::schema::CreateMoneyTransactionSchema;

#[derive(Debug)]
pub enum MoneyTransactionError {
    InvalidAmount,
    OtherError,
}

impl std::fmt::Display for MoneyTransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MoneyTransactionError::InvalidAmount => write!(f, "Invalid transaction amount"),
            MoneyTransactionError::OtherError => write!(f, "Other database-related error"),
        }
    }
}

impl std::error::Error for MoneyTransactionError {}

pub struct TransactionsRepo {
    pool: PgPool,
}

impl TransactionsRepo {
    pub fn new(pool: PgPool) -> Self {
        TransactionsRepo { pool }
    }

    pub async fn get_available_balance(&self) -> Result<i64, MoneyTransactionError> {
        todo!()
    }

    pub async fn get_all(&self) -> Result<Vec<MoneyTransactionModel>, MoneyTransactionError> {
        todo!()
    }

    pub async fn create(
        &self,
        item: CreateMoneyTransactionSchema,
    ) -> Result<MoneyTransactionModel, MoneyTransactionError> {
        todo!()
    }
}
