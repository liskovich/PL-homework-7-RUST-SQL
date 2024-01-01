#[cfg(test)]
use mockall::{mock, predicate::*};
use shared_db::repo::transactions_repo::MoneyTransactionError;
use uuid::Uuid;

use shared_db::model::{
    CreateMoneyTransactionModel, CreatePlatformModel, MoneyTransactionModel, OilPlatformModel,
    PlatformType, UpdatePlatformModel,
};
use shared_db::repo::platform_repo::OilPlatformError;

mock! {
    pub OilPlaftormRepo {
        async fn get_by_id(&self, id: Uuid) -> Result<OilPlatformModel, OilPlatformError>;
        async fn get_all(&self) -> Result<Vec<OilPlatformModel>, OilPlatformError>;
        async fn create(&self, item: CreatePlatformModel) -> Result<OilPlatformModel, OilPlatformError>;
        async fn update(&self, id: Uuid, new_item: UpdatePlatformModel) -> Result<OilPlatformModel, OilPlatformError>;
    }
}

#[tokio::test]
async fn test_get_platform_by_id() {
    let mut repo = MockOilPlaftormRepo::new();

    let expected_id = Uuid::new_v4();
    repo.expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(OilPlatformModel {
                id: id,
                platform_type: PlatformType::Ground,
                platform_level: 1,
                profitability: 10,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let result = repo.get_by_id(expected_id).await;
    assert!(result.is_ok());
    let platform = result.unwrap();
    assert_eq!(platform.id, expected_id);
    assert_eq!(platform.platform_type, PlatformType::Ground);
    assert_eq!(platform.platform_level, 1);
    assert_eq!(platform.profitability, 10);
    assert_eq!(platform.created_at, Some(12345));
    assert_eq!(platform.updated_at, Some(12345));
}

#[tokio::test]
async fn test_create_platform() {
    let mut repo = MockOilPlaftormRepo::new();

    let platform_to_create = CreatePlatformModel {
        platform_type: PlatformType::Rig,
        profitability: 100,
    };
    repo.expect_create()
        .with(eq(platform_to_create.clone()))
        .once()
        .returning(|item| {
            Ok(OilPlatformModel {
                id: Uuid::new_v4(),
                platform_type: item.platform_type,
                platform_level: 0,
                profitability: item.profitability,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    let result = repo.create(platform_to_create).await;
    assert!(result.is_ok());
    let platform = result.unwrap();
    assert_eq!(platform.platform_type, PlatformType::Rig);
    assert_eq!(platform.platform_level, 0);
    assert_eq!(platform.profitability, 100);
    assert_eq!(platform.created_at, Some(12345));
    assert_eq!(platform.updated_at, Some(12345));
}

#[tokio::test]
async fn test_get_all_platforms() {
    let mut repo = MockOilPlaftormRepo::new();
    repo.expect_get_all().once().returning(|| {
        Ok(vec![
            OilPlatformModel {
                id: Uuid::new_v4(),
                platform_type: PlatformType::Ground,
                platform_level: 1,
                profitability: 10,
                created_at: Some(12345),
                updated_at: Some(12345),
            },
            OilPlatformModel {
                id: Uuid::new_v4(),
                platform_type: PlatformType::Pump,
                platform_level: 2,
                profitability: 20,
                created_at: Some(54321),
                updated_at: Some(54321),
            },
        ])
    });

    let result = repo.get_all().await;
    assert!(result.is_ok());
    let platforms = result.unwrap();
    assert_eq!(platforms.len(), 2);

    let first = &platforms[0];
    assert_eq!(first.platform_type, PlatformType::Ground);
    assert_eq!(first.platform_level, 1);
    assert_eq!(first.profitability, 10);
    assert_eq!(first.created_at, Some(12345));
    assert_eq!(first.updated_at, Some(12345));
    let second = &platforms[1];
    assert_eq!(second.platform_type, PlatformType::Pump);
    assert_eq!(second.platform_level, 2);
    assert_eq!(second.profitability, 20);
    assert_eq!(second.created_at, Some(54321));
    assert_eq!(second.updated_at, Some(54321));
}

#[tokio::test]
async fn test_update_platform_success() {
    let mut repo = MockOilPlaftormRepo::new();

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 10,
    };
    let expected_id = Uuid::new_v4();
    repo.expect_update()
        .with(eq(expected_id), eq(platform_to_update.clone()))
        .once()
        .returning(|id, item| {
            Ok(OilPlatformModel {
                id: id,
                platform_type: PlatformType::Ground,
                platform_level: 2,
                profitability: 10 + item.profitability_addition,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let result = repo.update(expected_id, platform_to_update).await;
    assert!(result.is_ok());
    let platform = result.unwrap();
    assert_eq!(platform.platform_type, PlatformType::Ground);
    assert_eq!(platform.platform_level, 2);
    assert_eq!(platform.profitability, 20);
    assert_eq!(platform.created_at, Some(12345));
    assert_eq!(platform.updated_at, Some(12345));
}

#[tokio::test]
async fn test_update_platform_fail() {
    let mut repo = MockOilPlaftormRepo::new();

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 10,
    };
    let expected_id = Uuid::new_v4();
    repo.expect_update()
        .with(eq(expected_id), eq(platform_to_update.clone()))
        .once()
        .returning(|_, _| Err(OilPlatformError::MaxLevelReached));

    let result = repo.update(expected_id, platform_to_update).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(
        error.to_string(),
        "Maximum upgrade level of platform reached"
    );
}

// common transaction tests (for later use)
mock! {
    pub TransactionsRepo {
        async fn get_available_balance(&self) -> Result<i64, MoneyTransactionError>;
        async fn get_period_platform_earnings(&self) -> Result<i64, MoneyTransactionError>;
        async fn get_all(&self) -> Result<Vec<MoneyTransactionModel>, MoneyTransactionError>;
        async fn create(
            &self,
            item: CreateMoneyTransactionModel,
        ) -> Result<MoneyTransactionModel, MoneyTransactionError>;
    }
}

#[tokio::test]
async fn test_get_available_balance() {
    let mut repo = MockTransactionsRepo::new();

    repo.expect_get_available_balance()
        .once()
        .returning(|| Ok(100));

    let result = repo.get_available_balance().await;
    assert!(result.is_ok());
    let balance = result.unwrap();
    assert!(balance.is_positive());
    assert_eq!(balance, 100);
}

#[tokio::test]
async fn test_get_period_platform_earnings() {
    let mut repo = MockTransactionsRepo::new();

    repo.expect_get_period_platform_earnings()
        .once()
        .returning(|| Ok(100));

    let result = repo.get_period_platform_earnings().await;
    assert!(result.is_ok());
    let period_earnings = result.unwrap();
    assert!(period_earnings.is_positive());
    assert_eq!(period_earnings, 100);
}

#[tokio::test]
async fn test_get_all() {
    let mut repo = MockTransactionsRepo::new();

    repo.expect_get_all().once().returning(|| {
        Ok(vec![
            MoneyTransactionModel {
                id: Uuid::new_v4(),
                item_id: Uuid::new_v4(),
                amount: 100,
                reduces_balance: false,
                created_at: Some(12345),
                updated_at: Some(12345),
            },
            MoneyTransactionModel {
                id: Uuid::new_v4(),
                item_id: Uuid::new_v4(),
                amount: 10,
                reduces_balance: true,
                created_at: Some(54321),
                updated_at: Some(54321),
            },
        ])
    });

    // Test the get_all function
    let result = repo.get_all().await;
    assert!(result.is_ok());
    let transactions = result.unwrap();
    let first = &transactions[0];
    assert_eq!(first.amount, 100);
    assert_eq!(first.reduces_balance, false);
    assert_eq!(first.created_at, Some(12345));
    assert_eq!(first.updated_at, Some(12345));
    let second = &transactions[1];
    assert_eq!(second.amount, 10);
    assert_eq!(second.reduces_balance, true);
    assert_eq!(second.created_at, Some(54321));
    assert_eq!(second.updated_at, Some(54321));
}

#[tokio::test]
async fn test_create_increases_balance() {
    let mut repo = MockTransactionsRepo::new();

    repo.expect_get_available_balance()
        .once()
        .returning(|| Ok(100));
    let balance_before = repo.get_available_balance().await.unwrap();
    assert_eq!(balance_before, 100);

    let sample_model = CreateMoneyTransactionModel {
        item_id: None,
        amount: 100,
        reduces_balance: false,
    };
    repo.expect_create()
        .with(eq(sample_model.clone()))
        .once()
        .returning(move |tx| {
            Ok(MoneyTransactionModel {
                item_id: Uuid::nil(),
                amount: tx.amount,
                reduces_balance: tx.reduces_balance,
                id: Uuid::new_v4(),
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let result = repo.create(sample_model).await;
    assert!(result.is_ok());
    let transaction = result.unwrap();
    assert_eq!(transaction.amount, 100);
    assert_eq!(transaction.reduces_balance, false);
    assert_eq!(transaction.created_at, Some(12345));
    assert_eq!(transaction.updated_at, Some(12345));

    repo.expect_get_available_balance()
        .once()
        .returning(|| Ok(200));
    let balance_after = repo.get_available_balance().await.unwrap();
    assert_eq!(balance_after, 200);
}
