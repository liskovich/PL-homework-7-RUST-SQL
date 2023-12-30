use shared_db::model::{CreatePlatformModel, PlatformType, UpdatePlatformModel};
use shared_db::repo::{generic::Repo, platform_repo::OilPlaftormRepo};
use shared_db::test_util::get_test_pool;

// TODO: create mock database
async fn create_platform_repo() -> OilPlaftormRepo {
    let pool = get_test_pool().await;
    OilPlaftormRepo::new(pool)
}

#[tokio::test]
async fn test_create_get_platform_by_id() {
    let repo = create_platform_repo().await;

    let platform_to_create = CreatePlatformModel {
        platform_type: PlatformType::Rig,
        profitability: 100,
    };
    let created_platform = repo.create(platform_to_create.clone()).await.unwrap();

    let retrieved_platform = repo.get_by_id(created_platform.id).await.unwrap();
    assert_eq!(
        retrieved_platform.platform_type,
        platform_to_create.platform_type
    );
    assert_eq!(
        retrieved_platform.profitability,
        platform_to_create.profitability
    );
}

#[tokio::test]
async fn test_get_all_platforms() {
    let repo = create_platform_repo().await;
    let result = repo.get_all().await;
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 0);
}

#[tokio::test]
async fn test_update_platform() {
    let repo = create_platform_repo().await;

    // Create a platform and perform an update operation
    let platform_to_create = CreatePlatformModel {
        platform_type: PlatformType::Rig,
        profitability: 100,
    };
    let created_platform = repo.create(platform_to_create.clone()).await.unwrap();

    let platform_to_update = UpdatePlatformModel {
        profitability_addition: 10,
    };
    let updated_platform = repo
        .update(created_platform.id, platform_to_update)
        .await
        .unwrap();
    assert_eq!(
        updated_platform.platform_type,
        platform_to_create.platform_type
    );
    assert_eq!(
        updated_platform.profitability,
        platform_to_create.profitability + 10
    );
}
