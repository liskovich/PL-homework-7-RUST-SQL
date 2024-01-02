#[cfg(test)]
use mockall::predicate::*;
use rocket::http::{ContentType, Status};
use rocket_app::schema::{AppRepositories, BeerListResponse, GenericResponse, SingleBeerResponse};
use shared_db::model::BeerModel;
use shared_db::model::{CreateMoneyTransactionModel, MoneyTransactionModel};

use shared_db::repo::beer_repo::BeerError;
use uuid::Uuid;

use crate::test_util::{
    aget_rocket_client, MockBeerRepo, MockOilPlaftormRepo, MockTransactionsRepo,
};

#[rocket::async_test]
async fn test_beer_list_handler() {
    let platform_repo = MockOilPlaftormRepo::new();
    let mut beer_repo = MockBeerRepo::new();
    let finances_repo = MockTransactionsRepo::new();

    beer_repo.expect_get_all().once().returning(|| {
        Ok(vec![BeerModel {
            id: Uuid::new_v4(),
            title: "Beer 1".to_string(),
            description: "Description 1".to_string(),
            thumbnail: "img_url".to_string(),
            cost: 10,
            purchased: Some(false),
            created_at: Some(12345),
            updated_at: Some(12345),
        }])
    });

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client.get("/api/beers").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_beers: BeerListResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_beers.status, "success");
    assert_eq!(serialized_beers.results, 1);

    let beer = &serialized_beers.beers[0];
    assert_eq!(beer.title, "Beer 1");
    assert_eq!(beer.description, "Description 1");
    assert_eq!(beer.thumbnail, "img_url");
    assert_eq!(beer.cost, 10);
    assert_eq!(beer.purchased, Some(false));
    assert_eq!(beer.created_at, Some(12345));
    assert_eq!(beer.updated_at, Some(12345));
}

#[rocket::async_test]
async fn test_purchase_beer_handler_success() {
    let platform_repo = MockOilPlaftormRepo::new();
    let mut beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let expected_id = Uuid::new_v4();
    beer_repo
        .expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(BeerModel {
                id: id,
                title: "Beer 1".to_string(),
                description: "Description 1".to_string(),
                thumbnail: "img_url".to_string(),
                cost: 10,
                purchased: Some(false),
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    beer_repo
        .expect_purchase()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(BeerModel {
                id: id,
                title: "Beer 1".to_string(),
                description: "Description 1".to_string(),
                thumbnail: "img_url".to_string(),
                cost: 10,
                purchased: Some(true),
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(200));

    let tx = CreateMoneyTransactionModel {
        item_id: Some(expected_id),
        amount: 10,
        reduces_balance: true,
    };
    finances_repo
        .expect_create()
        .with(eq(tx))
        .once()
        .returning(|tx| {
            Ok(MoneyTransactionModel {
                id: Uuid::new_v4(),
                item_id: tx.item_id.unwrap(),
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
        .patch(format!("/api/beers/{}", expected_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_beer: SingleBeerResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_beer.status, "success");

    let beer = &serialized_beer.data;
    assert_eq!(beer.title, "Beer 1");
    assert_eq!(beer.description, "Description 1");
    assert_eq!(beer.thumbnail, "img_url");
    assert_eq!(beer.cost, 10);
    assert_eq!(beer.purchased, Some(true));
    assert_eq!(beer.created_at, Some(12345));
    assert_eq!(beer.updated_at, Some(12345));
}

#[rocket::async_test]
async fn test_purchase_beer_handler_insufficient_funds() {
    let platform_repo = MockOilPlaftormRepo::new();
    let mut beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let expected_id = Uuid::new_v4();
    beer_repo
        .expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(BeerModel {
                id: id,
                title: "Beer 1".to_string(),
                description: "Description 1".to_string(),
                thumbnail: "img_url".to_string(),
                cost: 1000,
                purchased: Some(false),
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(200));

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client
        .patch(format!("/api/beers/{}", expected_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_beer: GenericResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_beer.status, "error");
    assert_eq!(serialized_beer.message, "Not enough funds for purchase");
}

#[rocket::async_test]
async fn test_purchase_beer_handler_already_purchased() {
    let platform_repo = MockOilPlaftormRepo::new();
    let mut beer_repo = MockBeerRepo::new();
    let mut finances_repo = MockTransactionsRepo::new();

    let expected_id = Uuid::new_v4();
    beer_repo
        .expect_get_by_id()
        .with(eq(expected_id))
        .once()
        .returning(|id| {
            Ok(BeerModel {
                id: id,
                title: "Beer 1".to_string(),
                description: "Description 1".to_string(),
                thumbnail: "img_url".to_string(),
                cost: 10,
                purchased: Some(true),
                created_at: Some(12345),
                updated_at: Some(12345),
            })
        });
    beer_repo
        .expect_purchase()
        .with(eq(expected_id))
        .once()
        .returning(|_| Err(BeerError::AlreadyPurchased));
    finances_repo
        .expect_get_available_balance()
        .once()
        .returning(|| Ok(200));

    let repos = AppRepositories {
        platform_repo: Box::new(platform_repo),
        beer_repo: Box::new(beer_repo),
        finances_repo: Box::new(finances_repo),
    };
    let client = aget_rocket_client(repos).await;
    let response = client
        .patch(format!("/api/beers/{}", expected_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let serialized_beer: GenericResponse =
        serde_json::from_str(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(serialized_beer.status, "error");
    assert_eq!(
        serialized_beer.message,
        "You have already purchased this beer"
    );
}
