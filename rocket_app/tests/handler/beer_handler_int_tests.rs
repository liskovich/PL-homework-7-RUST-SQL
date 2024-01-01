#[cfg(test)]
use mockall::{mock, predicate::*};
use shared_db::model::BeerModel;

use shared_db::repo::beer_repo::BeerError;
use uuid::Uuid;

mock! {
    pub BeerRepo {
        pub async fn get_by_id(&self, id: Uuid) -> Result<BeerModel, BeerError>;
        pub async fn get_all(&self) -> Result<Vec<BeerModel>, BeerError>;
        pub async fn purchase(&self, id: Uuid) -> Result<BeerModel, BeerError>;
    }
}

#[tokio::test]
async fn test_get_all() {
    let mut beer_repo = MockBeerRepo::new();
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
    let result = beer_repo.get_all().await;
    assert!(result.is_ok());

    let beers = result.unwrap();
    let beer = &beers[0];
    assert_eq!(beer.title, "Beer 1");
    assert_eq!(beer.description, "Description 1");
    assert_eq!(beer.thumbnail, "img_url");
    assert_eq!(beer.cost, 10);
    assert_eq!(beer.purchased, Some(false));
    assert_eq!(beer.created_at, Some(12345));
    assert_eq!(beer.updated_at, Some(12345));
}

#[tokio::test]
async fn test_get_by_id_success() {
    let mut beer_repo = MockBeerRepo::new();
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
    let result = beer_repo.get_by_id(expected_id).await;
    assert!(result.is_ok());

    let beer = result.unwrap();
    assert_eq!(beer.id, expected_id);
    assert_eq!(beer.title, "Beer 1");
    assert_eq!(beer.description, "Description 1");
    assert_eq!(beer.thumbnail, "img_url");
    assert_eq!(beer.cost, 10);
    assert_eq!(beer.purchased, Some(false));
    assert_eq!(beer.created_at, Some(12345));
    assert_eq!(beer.updated_at, Some(12345));
}

#[tokio::test]
async fn test_purchase_success() {
    let mut beer_repo = MockBeerRepo::new();
    let expected_id = Uuid::new_v4();
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

    let result = beer_repo.purchase(expected_id).await;
    assert!(result.is_ok());
    let beer = result.unwrap();
    assert_eq!(beer.id, expected_id);
    assert_eq!(beer.title, "Beer 1");
    assert_eq!(beer.description, "Description 1");
    assert_eq!(beer.thumbnail, "img_url");
    assert_eq!(beer.cost, 10);
    assert_eq!(beer.purchased, Some(true));
    assert_eq!(beer.created_at, Some(12345));
    assert_eq!(beer.updated_at, Some(12345));
}

#[tokio::test]
async fn test_purchase_fail() {
    let mut beer_repo = MockBeerRepo::new();
    let already_purchased_id = Uuid::new_v4();
    beer_repo
        .expect_purchase()
        .with(eq(already_purchased_id))
        .once()
        .returning(|_| Err(BeerError::AlreadyPurchased));

    let result = beer_repo.purchase(already_purchased_id).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.to_string(), "Beer already purchased");
}
