use std::str::FromStr;

use shared_db::repo::beer_repo::{BeerRepo, BeerRepoTrait};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("beers"))]
async fn test_get_all(pool: PgPool) -> sqlx::Result<()> {
    let beer_repo = BeerRepo::new(pool);
    let result = beer_repo.get_all().await;
    assert!(result.is_ok());
    let beers = result.unwrap();
    assert_eq!(beers.len(), 4);
    Ok(())
}

#[sqlx::test(fixtures("beers"))]
async fn test_get_by_id_success(pool: PgPool) -> sqlx::Result<()> {
    let beer_repo = BeerRepo::new(pool);
    let beer_id = Uuid::from_str("8983e5a3-f6b5-40c5-81a2-c02a498116fc").unwrap();
    let result = beer_repo.get_by_id(beer_id).await;
    assert!(result.is_ok());
    let beer = result.unwrap();
    assert_eq!(beer.title, "Beer 1");
    assert_eq!(beer.description, "Description 1");
    assert_eq!(beer.thumbnail, "img_1.png");
    assert_eq!(beer.cost, 5);
    assert_eq!(beer.purchased, Some(true));
    Ok(())
}

#[sqlx::test(fixtures("beers"))]
async fn test_get_by_id_fail(pool: PgPool) -> sqlx::Result<()> {
    let beer_repo = BeerRepo::new(pool);
    // beer with such id does not exist
    let beer_id = Uuid::new_v4();
    let result = beer_repo.get_by_id(beer_id).await;
    assert!(result.is_err());
    Ok(())
}

#[sqlx::test(fixtures("beers"))]
async fn test_purchase_success(pool: PgPool) -> sqlx::Result<()> {
    let beer_repo = BeerRepo::new(pool);
    let beer_id = Uuid::from_str("effaf31e-aedc-42de-adce-322d73ef69d0").unwrap();
    let beer = beer_repo.get_by_id(beer_id).await.unwrap();
    assert_eq!(beer.purchased, Some(false));
    let result = beer_repo.purchase(beer_id).await;
    assert!(result.is_ok());
    let beer = beer_repo.get_by_id(beer_id).await.unwrap();
    assert_eq!(beer.purchased, Some(true));
    Ok(())
}

#[sqlx::test(fixtures("beers"))]
async fn test_purchase_fail(pool: PgPool) -> sqlx::Result<()> {
    let beer_repo = BeerRepo::new(pool);
    let beer_id = Uuid::from_str("8983e5a3-f6b5-40c5-81a2-c02a498116fc").unwrap();
    let beer = beer_repo.get_by_id(beer_id).await.unwrap();
    assert_eq!(beer.purchased, Some(true));
    let result = beer_repo.purchase(beer_id).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Beer already purchased");
    Ok(())
}
