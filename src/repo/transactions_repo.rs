use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{CreateMoneyTransactionModel, MoneyTransactionModel};

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
        let query_result =
            match sqlx::query_as!(MoneyTransactionModel, "SELECT * FROM money_transactions")
                .fetch_all(&self.pool)
                .await
            {
                Ok(platforms) => platforms,
                Err(_) => return Err(MoneyTransactionError::OtherError),
            };

        let mut calculated_value: i64 = 0;
        for transaction in &query_result {
            if transaction.reduces_balance {
                calculated_value -= transaction.amount;
            } else {
                calculated_value += transaction.amount;
            }
        }
        Ok(calculated_value)
    }

    pub async fn get_all(&self) -> Result<Vec<MoneyTransactionModel>, MoneyTransactionError> {
        let query_result =
            match sqlx::query_as!(MoneyTransactionModel, "SELECT * FROM money_transactions")
                .fetch_all(&self.pool)
                .await
            {
                Ok(platforms) => platforms,
                Err(_) => return Err(MoneyTransactionError::OtherError),
            };
        Ok(query_result)
    }

    pub async fn create(
        &self,
        item: CreateMoneyTransactionModel,
    ) -> Result<MoneyTransactionModel, MoneyTransactionError> {
        if &item.amount < &0 {
            return Err(MoneyTransactionError::InvalidAmount);
        }

        let query_result = match sqlx::query_as!(
            MoneyTransactionModel,
            "INSERT INTO money_transactions (item_id, amount, reduces_balance) VALUES ($1, $2, $3) RETURNING *",
            item.item_id.unwrap_or_else(|| Uuid::nil()),
            item.amount,
            item.reduces_balance,
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(tx) => tx,
            Err(_) => return Err(MoneyTransactionError::OtherError),
        };

        Ok(query_result)
    }
}
