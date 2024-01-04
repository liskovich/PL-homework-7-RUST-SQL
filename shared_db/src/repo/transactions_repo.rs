use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{CreateMoneyTransactionModel, MoneyTransactionModel, NumericHandler};

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

#[async_trait]
pub trait TransactionsRepoTrait: Send + Sync {
    async fn get_available_balance(&self) -> Result<i64, MoneyTransactionError>;
    async fn get_period_platform_earnings(&self) -> Result<i64, MoneyTransactionError>;
    async fn get_all(&self) -> Result<Vec<MoneyTransactionModel>, MoneyTransactionError>;
    async fn create(
        &self,
        item: CreateMoneyTransactionModel,
    ) -> Result<MoneyTransactionModel, MoneyTransactionError>;
}

pub struct TransactionsRepo {
    pool: PgPool,
}

impl TransactionsRepo {
    pub fn new(pool: PgPool) -> Self {
        TransactionsRepo { pool }
    }
}

#[async_trait]
impl TransactionsRepoTrait for TransactionsRepo {
    /// Get available currency balance
    ///
    /// Returns the difference between all transactions that increase balance and those that reduce it from the database 'money_transactions' table.
    async fn get_available_balance(&self) -> Result<i64, MoneyTransactionError> {
        let query_result =
            match sqlx::query_as!(NumericHandler, "SELECT CAST(SUM(CASE WHEN reduces_balance = FALSE THEN amount ELSE -amount END) AS DECIMAL) AS calculation FROM money_transactions")
                .fetch_one(&self.pool)
                .await
            {
                Ok(balance) => balance,
                Err(_) => return Err(MoneyTransactionError::OtherError),
            };

        // convert from BigDecimal to i64
        let balance: Option<i64> = query_result
            .calculation
            .map(|bd| bd.to_string().parse().unwrap_or(0));
        match balance {
            Some(profit) => Ok(profit),
            None => Err(MoneyTransactionError::OtherError),
        }
    }

    /// Get total profitability
    ///
    /// Returns the sum of profitability of all oil platforms from the database 'oil_platforms' table.
    async fn get_period_platform_earnings(&self) -> Result<i64, MoneyTransactionError> {
        let query_result = match sqlx::query_as!(
            NumericHandler,
            "SELECT SUM(profitability) AS calculation FROM oil_platforms",
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(earnings) => earnings,
            Err(_) => return Err(MoneyTransactionError::OtherError),
        };

        // convert from BigDecimal to i64
        let total_profit: Option<i64> = query_result
            .calculation
            .map(|bd| bd.to_string().parse().unwrap_or(0));
        match total_profit {
            Some(profit) => Ok(profit),
            None => Err(MoneyTransactionError::OtherError),
        }
    }

    /// Get all money transactions
    ///
    /// Retrieves a list of all transactions from the database 'money_transactions' table.
    async fn get_all(&self) -> Result<Vec<MoneyTransactionModel>, MoneyTransactionError> {
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

    /// Create a money transaction
    ///
    /// Given the CreateMoneyTransactionModel which contains related item identifier, currency amount
    /// and whether it reduces currency balance, saves it to the database 'money_transactions' table.
    async fn create(
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
