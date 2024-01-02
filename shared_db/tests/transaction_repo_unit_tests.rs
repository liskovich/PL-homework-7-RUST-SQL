use sqlx::PgPool;

use shared_db::model::CreateMoneyTransactionModel;
use shared_db::repo::transactions_repo::{TransactionsRepo, TransactionsRepoTrait};

#[sqlx::test(fixtures("transactions"))]
async fn test_get_available_balance(pool: PgPool) -> sqlx::Result<()> {
    let repo = TransactionsRepo::new(pool);
    let result = repo.get_available_balance().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 90);
    Ok(())
}

#[sqlx::test(fixtures("transactions", "platforms"))]
async fn test_get_period_platform_earnings(pool: PgPool) -> sqlx::Result<()> {
    let repo = TransactionsRepo::new(pool);
    let result = repo.get_period_platform_earnings().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 150);
    Ok(())
}

#[sqlx::test(fixtures("transactions"))]
async fn test_get_all(pool: PgPool) -> sqlx::Result<()> {
    let repo = TransactionsRepo::new(pool);
    let result = repo.get_all().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
    Ok(())
}

#[sqlx::test(fixtures("transactions"))]
async fn test_create_increases_balance(pool: PgPool) -> sqlx::Result<()> {
    let repo = TransactionsRepo::new(pool);
    let transaction_count_before = repo.get_all().await.unwrap().len();
    let balance_before = repo.get_available_balance().await.unwrap();

    let sample_model = CreateMoneyTransactionModel {
        item_id: None,
        amount: 100,
        reduces_balance: false,
    };
    let result = repo.create(sample_model).await;
    assert!(result.is_ok());

    let transaction_count_after = repo.get_all().await.unwrap().len();
    let balance_after = repo.get_available_balance().await.unwrap();
    assert_eq!(transaction_count_after, transaction_count_before + 1);
    assert_eq!(balance_after, balance_before + 100);
    Ok(())
}

#[sqlx::test(fixtures("transactions"))]
async fn test_create_decreases_balance(pool: PgPool) -> sqlx::Result<()> {
    let repo = TransactionsRepo::new(pool);
    let transaction_count_before = repo.get_all().await.unwrap().len();
    let balance_before = repo.get_available_balance().await.unwrap();

    let sample_model = CreateMoneyTransactionModel {
        item_id: None,
        amount: 50,
        reduces_balance: true,
    };
    let result = repo.create(sample_model).await;
    assert!(result.is_ok());

    let transaction_count_after = repo.get_all().await.unwrap().len();
    let balance_after = repo.get_available_balance().await.unwrap();
    assert_eq!(transaction_count_after, transaction_count_before + 1);
    assert_eq!(balance_after, balance_before - 50);
    Ok(())
}
