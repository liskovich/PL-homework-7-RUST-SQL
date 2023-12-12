use rocket::get;
use rocket::tokio::time::{interval, Duration};
use rocket_ws::{Channel, Message, WebSocket};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::mpsc;
use uuid::Uuid;

use shared_db::{model::CreateMoneyTransactionModel, repo::transactions_repo::TransactionsRepo};

#[derive(Debug, Serialize, Deserialize)]
struct BalanceData {
    balance: i64,
    just_earned: i64,
}

#[get("/game-state")]
pub fn game_earnings_stream(ws: WebSocket) -> Channel<'static> {
    use rocket::futures::SinkExt;

    let (data_sender, mut data_receiver) = mpsc::channel(100);
    tokio::spawn(get_realtime_financial_data(data_sender));

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(data) = data_receiver.recv().await {
                let _ = stream.send(Message::Text(data)).await;
            }
            Ok(())
        })
    })
}

async fn get_realtime_financial_data(sender: mpsc::Sender<String>) {
    let mut interval = interval(Duration::from_secs(10));

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

    loop {
        interval.tick().await;

        // calculate earnings for this period
        let recent_earnings = match finance_repo.get_period_platform_earnings().await {
            Ok(result) => result,
            Err(_) => 0,
        };

        // TODO: update balance
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

        // send response to client
        let response_data = BalanceData {
            balance: current_balance,
            just_earned: recent_earnings,
        };
        let response_data = match serde_json::to_string(&response_data) {
            Ok(res) => res,
            Err(e) => format!("Failed to parse: {}", e.to_string()),
        };

        if sender.send(response_data).await.is_err() {
            break;
        }
    }
}
