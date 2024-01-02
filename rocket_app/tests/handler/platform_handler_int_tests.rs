#[cfg(test)]
use mockall::predicate::*;
use rocket::http::{ContentType, Status};
use rocket_app::schema::{
    AppRepositories, CreatePlatformSchema, GenericResponse, PlatformListResponse,
    SinglePlatformResponse,
};
use uuid::Uuid;

use shared_db::model::{
    CreateMoneyTransactionModel, CreatePlatformModel, MoneyTransactionModel, OilPlatformModel,
    PlatformType, UpdatePlatformModel,
};
use shared_db::repo::platform_repo::OilPlatformError;

use crate::test_util::{
    aget_rocket_client, MockBeerRepo, MockOilPlaftormRepo, MockTransactionsRepo,
};

#[rocket::async_test]
async fn test_platforms_list_handler() {
    let mut platform_repo = MockOilPlaftormRepo::new();
    let beer_repo = MockBeerRepo::new();
    let finances_repo = MockTransactionsRepo::new();

    platform_repo.expect_get_all().once().returning(|| {
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

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client.get("/api/platforms").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_platforms: PlatformListResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_platforms.status, "success");
    assert_eq!(serialized_platforms.results, 2);

    let first = &serialized_platforms.platforms[1];
    assert_eq!(first.platform_type, PlatformType::Pump);
    assert_eq!(first.platform_level, 2);
    assert_eq!(first.profitability, 20);
    assert_eq!(first.created_at, Some(54321));
    assert_eq!(first.updated_at, Some(54321));
    let second = &serialized_platforms.platforms[0];
    assert_eq!(second.platform_type, PlatformType::Ground);
    assert_eq!(second.platform_level, 1);
    assert_eq!(second.profitability, 10);
    assert_eq!(second.created_at, Some(12345));
    assert_eq!(second.updated_at, Some(12345));
}

#[rocket::async_test]
async fn test_create_platform_handler_success() {
    let mut platform_repo = MockOilPlaftormRepo::new();
    let beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let platform_to_create = CreatePlatformModel {
        platform_type: PlatformType::Rig,
        profitability: 5,
    };
    platform_repo
        .expect_create()
        .with(eq(platform_to_create))
        .once()
        .returning(|item| {
            Ok(OilPlatformModel {
                id: Uuid::nil(),
                platform_type: item.platform_type,
                platform_level: 0,
                profitability: item.profitability,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(2000));

    let tx = CreateMoneyTransactionModel {
        item_id: Some(Uuid::nil()),
        amount: 1000,
        reduces_balance: true,
    };
    finances_repo
        .expect_create()
        .with(eq(tx))
        .once()
        .returning(|tx| {
            Ok(MoneyTransactionModel {
                id: Uuid::new_v4(),
                item_id: Uuid::nil(),
                amount: tx.amount,
                reduces_balance: tx.reduces_balance,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let create_platform_request = CreatePlatformSchema {
        platform_type: "Rig".to_string(),
    };
    let response = client
        .post("/api/platforms")
        .json(&create_platform_request)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_platform: SinglePlatformResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_platform.status, "success");

    let platform = &serialized_platform.data;
    assert_eq!(platform.platform_type, PlatformType::Rig);
    assert_eq!(platform.platform_level, 0);
    assert_eq!(platform.profitability, 5);
    assert_eq!(platform.created_at, Some(12345));
    assert_eq!(platform.updated_at, Some(12345));
}

#[rocket::async_test]
async fn test_create_platform_handler_insufficient_funds() {
    let platform_repo = MockOilPlaftormRepo::new();
    let beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(500));

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let create_platform_request = CreatePlatformSchema {
        platform_type: "Rig".to_string(),
    };
    let response = client
        .post("/api/platforms")
        .json(&create_platform_request)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_platform: GenericResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_platform.status, "error");
    assert_eq!(serialized_platform.message, "Not enough funds for purchase");
}

#[rocket::async_test]
async fn test_edit_platform_handler_success() {
    let mut platform_repo = MockOilPlaftormRepo::new();
    let beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let expected_id = Uuid::nil();
    platform_repo
        .expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(OilPlatformModel {
                id: id,
                platform_type: PlatformType::Rig,
                platform_level: 0,
                profitability: 5,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 5,
    };
    platform_repo
        .expect_update()
        .with(eq(expected_id), eq(platform_to_update))
        .once()
        .returning(|id, item| {
            Ok(OilPlatformModel {
                id: id,
                platform_type: PlatformType::Rig,
                platform_level: 1,
                profitability: 5 + item.profitability_addition,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(2000));

    let tx = CreateMoneyTransactionModel {
        item_id: Some(Uuid::nil()),
        amount: 100,
        reduces_balance: true,
    };
    finances_repo
        .expect_create()
        .with(eq(tx))
        .once()
        .returning(|tx| {
            Ok(MoneyTransactionModel {
                id: Uuid::new_v4(),
                item_id: Uuid::nil(),
                amount: tx.amount,
                reduces_balance: tx.reduces_balance,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client
        .patch(format!("/api/platforms/{}", expected_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_platform: SinglePlatformResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_platform.status, "success");

    let platform = &serialized_platform.data;
    assert_eq!(platform.platform_type, PlatformType::Rig);
    assert_eq!(platform.platform_level, 1);
    assert_eq!(platform.profitability, 10);
    assert_eq!(platform.created_at, Some(12345));
    assert_eq!(platform.updated_at, Some(12345));
}

#[rocket::async_test]
async fn test_edit_platform_handler_max_level() {
    let mut platform_repo = MockOilPlaftormRepo::new();
    let beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let expected_id = Uuid::nil();
    platform_repo
        .expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(OilPlatformModel {
                id: id,
                platform_type: PlatformType::Rig,
                platform_level: 0,
                profitability: 5,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 5,
    };
    platform_repo
        .expect_update()
        .with(eq(expected_id), eq(platform_to_update))
        .once()
        .returning(|_, _| Err(OilPlatformError::MaxLevelReached));
    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(2000));

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client
        .patch(format!("/api/platforms/{}", expected_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_platform: GenericResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_platform.status, "error");
    assert_eq!(
        serialized_platform.message,
        "You have already upgraded the platform to the maximum"
    );
}

#[rocket::async_test]
async fn test_edit_platform_handler_insufficient_funds() {
    let mut platform_repo = MockOilPlaftormRepo::new();
    let beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let expected_id = Uuid::nil();
    platform_repo
        .expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(OilPlatformModel {
                id: id,
                platform_type: PlatformType::Rig,
                platform_level: 0,
                profitability: 5,
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });

    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(20));

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client
        .patch(format!("/api/platforms/{}", expected_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_platform: GenericResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_platform.status, "error");
    assert_eq!(serialized_platform.message, "Not enough funds for purchase");
}
