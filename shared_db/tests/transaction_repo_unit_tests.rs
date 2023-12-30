use shared_db::model::CreateMoneyTransactionModel;
use shared_db::repo::transactions_repo::TransactionsRepo;
use shared_db::test_util::get_test_pool;

// TODO: create mock database
async fn create_transaction_repo() -> TransactionsRepo {
    let pool = get_test_pool().await;
    TransactionsRepo::new(pool)
}

#[tokio::test]
async fn test_get_available_balance() {
    let repo = create_transaction_repo().await;

    // Test the get_available_balance function
    let result = repo.get_available_balance().await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_positive());
}

#[tokio::test]
async fn test_get_period_platform_earnings() {
    let repo = create_transaction_repo().await;

    // Test the get_period_platform_earnings function
    let result = repo.get_period_platform_earnings().await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_positive());
}

#[tokio::test]
async fn test_get_all() {
    let repo = create_transaction_repo().await;

    // Test the get_all function
    let result = repo.get_all().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_increases_balance() {
    let repo = create_transaction_repo().await;
    let transaction_count_before = repo.get_all().await.unwrap().len();
    let balance_before = repo.get_available_balance().await.unwrap();

    // Create a sample CreateMoneyTransactionModel instance
    let sample_model = CreateMoneyTransactionModel {
        item_id: None,
        amount: 100,
        reduces_balance: false,
    };

    // Test the create function
    let result = repo.create(sample_model).await;
    assert!(result.is_ok());

    // Test overall balance
    let transaction_count_after = repo.get_all().await.unwrap().len();
    let balance_after = repo.get_available_balance().await.unwrap();
    assert_eq!(transaction_count_after, transaction_count_before + 1);
    assert_eq!(balance_after, balance_before + 100);
}
