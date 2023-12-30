use std::str::FromStr;

use shared_db::test_util::get_test_pool;
use shared_db::repo::beer_repo::BeerRepo;
use uuid::Uuid;

// TODO: create mock database
async fn create_beer_repo() -> BeerRepo {
    let pool = get_test_pool().await;
    BeerRepo::new(pool)
}

#[tokio::test]
async fn test_get_all() {
    let beer_repo = create_beer_repo().await;
    let result = beer_repo.get_all().await;
    assert!(result.is_ok());
    let beers = result.unwrap();
    assert!(beers.len() == 8);
}

#[tokio::test]
async fn test_get_by_id_success() {
    let beer_repo = create_beer_repo().await;
    let beer_id = Uuid::from_str("d4602efa-df38-447f-8da6-d6ccecbd07fa").unwrap();
    let result = beer_repo.get_by_id(beer_id).await;
    assert!(result.is_ok());
    let beer = result.unwrap();
    assert!(beer.title == "Heineken");
    assert!(beer.cost == 15000);
}

#[tokio::test]
async fn test_get_by_id_fail() {
    let beer_repo = create_beer_repo().await;
    // beer with such id does not exist
    let beer_id = Uuid::new_v4();
    let result = beer_repo.get_by_id(beer_id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_purchase_success() {
    let beer_repo = create_beer_repo().await;
    // beer with such id does not exist
    let beer_id = Uuid::from_str("d4602efa-df38-447f-8da6-d6ccecbd07fa").unwrap();
    let beer = beer_repo.get_by_id(beer_id).await.unwrap();
    println!("{:?}", beer.purchased);
    assert!(beer.purchased == Some(false));
    let result = beer_repo.purchase(beer_id).await;
    assert!(result.is_ok());
    let beer = beer_repo.get_by_id(beer_id).await.unwrap();
    assert!(beer.purchased == Some(true));
}

#[tokio::test]
async fn test_purchase_fail() {
    let beer_repo = create_beer_repo().await;
    // beer with such id does not exist
    let beer_id = Uuid::new_v4();
    let beer = beer_repo.get_by_id(beer_id).await;
    assert!(beer.is_err());
    let result = beer_repo.purchase(beer_id).await;
    assert!(result.is_err());
}
