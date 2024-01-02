use serde::{Deserialize, Serialize};
use shared_db::{model::CreateMoneyTransactionModel, repo::transactions_repo::{TransactionsRepo, TransactionsRepoTrait}};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct BalanceData {
    balance: i64,
    just_earned: i64,
}

pub async fn get_realtime_financial_data() -> String {
    // separate database connection for streams
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let finance_repo = TransactionsRepo::new(pool);

    // calculate earnings for this period
    let recent_earnings = match finance_repo.get_period_platform_earnings().await {
        Ok(result) => result,
        Err(_) => 0,
    };

    // update balance
    let earnings_tx = CreateMoneyTransactionModel {
        item_id: Some(Uuid::nil()),
        amount: recent_earnings,
        reduces_balance: false,
    };
    let _ = match finance_repo.create(earnings_tx).await {
        Ok(_) => true,
        Err(_) => false,
    };

    // retrieve updated balance
    let current_balance = match finance_repo.get_available_balance().await {
        Ok(result) => result,
        Err(_) => 0,
    };

    let response_data = BalanceData {
        balance: current_balance,
        just_earned: recent_earnings,
    };
    match serde_json::to_string(&response_data) {
        Ok(res) => res,
        Err(_) => "".to_string(),
    }
}
