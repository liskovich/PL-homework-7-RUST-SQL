use std::panic;

use crate::{
    repo::generic::Repo,
    schema::{CreatePlatformSchema, GenericResponse, PlatformListResponse, SinglePlatformResponse}, AppRepositories, model::PlatformType,
};
use rocket::{get, http::Status, post, response::status::Custom, serde::json::Json, State};

#[get("/health")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: "service alive".to_string(),
    };
    Ok(Json(response_json))
}

#[get("/platforms")]
pub async fn platforms_list_handler(
    data: &State<AppRepositories>,
) -> Result<Json<PlatformListResponse>, Status> {
    let oil_platform_repo = &data.platform_repo;

    let platforms = oil_platform_repo
        .get_all()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let json_response = PlatformListResponse {
        status: "success".to_string(),
        results: platforms.len(),
        platforms,
    };
    Ok(Json(json_response))
}

#[post("/platforms", data = "<body>")]
pub async fn create_todo_handler(
    body: Json<CreatePlatformSchema>,
    data: &State<AppRepositories>,
) -> Result<Json<SinglePlatformResponse>, Custom<Json<GenericResponse>>> {
    let platform_create = body.into_inner();

    // validate platform type
    let platform_type_ref: &str = &platform_create.platform_type;
    let result = panic::catch_unwind(|| {
        let validate_platform_type: PlatformType = platform_type_ref.into();
        println!("Platform enum: {:?}", validate_platform_type);
    });
    if let Err(e) = result {
        println!("Panic occurred: {:?}", e);
        let error_response = Custom(Status::BadRequest, Json(GenericResponse {
            status: "error".to_string(),
            message: "Invalid platform type specified".to_string(),
        }));
        return Err(error_response)
    }

    // TODO: validate amount of available money

    // save to database
    let oil_platform_repo = &data.platform_repo;
    match oil_platform_repo.create(platform_create).await {
        Ok(platform) => {
            let json_response = SinglePlatformResponse {
                status: "success".to_string(),
                data: platform,
            };
            Ok(Json(json_response))
        }
        Err(_) => {
            let error_response = Custom(Status::InternalServerError, Json(GenericResponse {
                status: "error".to_string(),
                message: "Failed to create platform".to_string(),
            }));
            Err(error_response)
        }
    }
}
