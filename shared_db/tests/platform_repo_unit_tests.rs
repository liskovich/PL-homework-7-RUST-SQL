use std::str::FromStr;

use shared_db::model::{CreatePlatformModel, PlatformType, UpdatePlatformModel};
use shared_db::repo::{generic::Repo, platform_repo::OilPlaftormRepo};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("platforms"))]
async fn test_get_platform_by_id(pool: PgPool) -> sqlx::Result<()> {
    let repo = OilPlaftormRepo::new(pool);
    let retrieved_platform = repo
        .get_by_id(Uuid::from_str("a63169d5-4107-4a0a-9d82-dad11b6500b3").unwrap())
        .await
        .unwrap();
    assert_eq!(retrieved_platform.platform_type, PlatformType::Rig);
    assert_eq!(retrieved_platform.platform_level, 0);
    assert_eq!(retrieved_platform.profitability, 10);
    Ok(())
}

#[sqlx::test(fixtures("platforms"))]
async fn test_create_platform(pool: PgPool) -> sqlx::Result<()> {
    let repo = OilPlaftormRepo::new(pool);
    let platform_to_create = CreatePlatformModel {
        platform_type: PlatformType::Rig,
        profitability: 100,
    };
    let created_platform = repo.create(platform_to_create.clone()).await.unwrap();

    assert_eq!(
        created_platform.platform_type,
        platform_to_create.platform_type
    );
    assert_eq!(created_platform.platform_level, 0);
    assert_eq!(
        created_platform.profitability,
        platform_to_create.profitability
    );
    Ok(())
}

#[sqlx::test(fixtures("platforms"))]
async fn test_get_all_platforms(pool: PgPool) -> sqlx::Result<()> {
    let repo = OilPlaftormRepo::new(pool);
    let result = repo.get_all().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 4);
    Ok(())
}

#[sqlx::test(fixtures("platforms"))]
async fn test_update_platform_success(pool: PgPool) -> sqlx::Result<()> {
    let repo = OilPlaftormRepo::new(pool);
    let update_id = Uuid::from_str("226d0256-e280-4262-ab57-b1e30129805d").unwrap();
    let platform_before = repo.get_by_id(update_id).await;
    assert!(platform_before.is_ok());
    let platform_before = platform_before.unwrap();

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 10,
    };
    let updated_platform = repo.update(update_id, platform_to_update).await;
    assert!(updated_platform.is_ok());
    let updated_platform = updated_platform.unwrap();
    assert_eq!(
        platform_before.platform_type,
        updated_platform.platform_type
    );
    assert_eq!(
        platform_before.platform_level + 1,
        updated_platform.platform_level
    );
    assert_eq!(
        platform_before.profitability + 10,
        updated_platform.profitability
    );
    Ok(())
}

#[sqlx::test(fixtures("platforms"))]
async fn test_update_platform_fail(pool: PgPool) -> sqlx::Result<()> {
    let repo = OilPlaftormRepo::new(pool);
    let update_id = Uuid::from_str("fb4bcfa2-dd9b-4b17-95b1-f5bebf9d6e30").unwrap();
    let platform_before = repo.get_by_id(update_id).await;
    assert!(platform_before.is_ok());

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 10,
    };
    let updated_platform = repo.update(update_id, platform_to_update).await;
    assert!(updated_platform.is_err());
    let error = updated_platform.unwrap_err();
    assert_eq!(
        error.to_string(),
        "Maximum upgrade level of platform reached"
    );
    Ok(())
}
