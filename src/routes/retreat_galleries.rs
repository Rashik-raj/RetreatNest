use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::Response,
    routing::{delete, get, patch, post},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
    TryIntoModel,
};
use validator::Validate;

use crate::{
    entities_helper::{
        RetreatColumn, RetreatEntity, RetreatGalleriesActiveModel, RetreatGalleriesColumn,
        RetreatGalleriesEntity, RetreatGalleriesModel,
    },
    serializers::{
        retreat_galleries::ReadRetreatGallerySerializer,
        retreat_reviews::UpdateRetreatReviewSerializer,
    },
    state::AppState,
    utils::{
        extractors::auth::AuthUser,
        response::{CustomResponse, to_error_response, to_error_response_with_message},
        storage::store_retreat_gallery,
    },
};

async fn create_retreat_gallery(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(retreat_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Response<Body>, Response<Body>> {
    // Find existing Retreat
    RetreatEntity::find()
        .filter(RetreatColumn::RetreatId.eq(retreat_id))
        .one(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            to_error_response_with_message("Retreat not found.", StatusCode::NOT_FOUND)
        })?;

    let mut caption: Option<String> = None;
    let mut image_path: String = "".to_string();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "caption" => {
                if let Ok(text) = field.text().await {
                    caption = Some(text);
                }
            }
            "image" => {
                let file_name: String = field.file_name().unwrap().to_string();
                let file_content: Bytes = field.bytes().await.unwrap();
                image_path = store_retreat_gallery(file_content, file_name).await;
            }
            _ => {}
        }
    }
    let active_model: RetreatGalleriesActiveModel = RetreatGalleriesActiveModel {
        caption: Set(caption),
        image_path: Set(image_path),
        retreat_id: Set(retreat_id),
        created_by: Set(Some(user.user_id)),
        updated_by: Set(Some(user.user_id)),
        ..Default::default()
    };

    // save user
    let active_model: RetreatGalleriesActiveModel = active_model
        .save(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    // convert to ReadUserSerializer serializer
    let serializer: ReadRetreatGallerySerializer = active_model
        .try_into_model()
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?
        .into();

    Ok(CustomResponse::builder(serializer)
        .status_code(StatusCode::CREATED)
        .build())
}

async fn list_retreat_gallery(
    State(state): State<AppState>,
    Path(retreat_id): Path<i64>,
) -> Result<Response<Body>, Response<Body>> {
    // Find existing Retreat
    RetreatEntity::find()
        .filter(RetreatColumn::RetreatId.eq(retreat_id))
        .one(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            to_error_response_with_message("Retreat not found.", StatusCode::NOT_FOUND)
        })?;

    let instances: Vec<RetreatGalleriesModel> = RetreatGalleriesEntity::find()
        .filter(RetreatGalleriesColumn::RetreatId.eq(retreat_id))
        .all(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    // Convert model to serializer
    let serializers: Vec<ReadRetreatGallerySerializer> =
        instances.into_iter().map(|model| model.into()).collect();

    Ok(CustomResponse::builder(serializers).build())
}

async fn update_retreat_gallery(
    State(state): State<AppState>,
    AuthUser(_): AuthUser,
    Path((retreat_id, gallery_id)): Path<(i64, i64)>,
    Json(payload): Json<UpdateRetreatReviewSerializer>,
) -> Result<Response<Body>, Response<Body>> {
    payload
        .validate()
        .map_err(|e| to_error_response(e, StatusCode::BAD_REQUEST))?;

    RetreatEntity::find()
        .filter(RetreatColumn::RetreatId.eq(retreat_id))
        .one(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            to_error_response_with_message("Retreat not found.", StatusCode::NOT_FOUND)
        })?;

    let instance: RetreatGalleriesModel = RetreatGalleriesEntity::find()
        .filter(RetreatGalleriesColumn::GalleryId.eq(gallery_id))
        .filter(RetreatGalleriesColumn::RetreatId.eq(retreat_id))
        .one(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            to_error_response_with_message("Retreat gallery not found.", StatusCode::NOT_FOUND)
        })?;

    // Convert to ActiveModel for editing
    let mut active_model: RetreatGalleriesActiveModel = instance.into_active_model();

    // set_fields!(active_model, payload, rating, review);

    // Save the updated Retreat
    let instance: RetreatGalleriesModel = active_model
        .update(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    // Convert to serializer
    let serializer: ReadRetreatGallerySerializer = instance.into();

    Ok(CustomResponse::builder(serializer).build())
}

async fn delete_retreat_gallery(
    State(state): State<AppState>,
    AuthUser(_): AuthUser,
    Path((retreat_id, gallery_id)): Path<(i64, i64)>,
) -> Result<Response<Body>, Response<Body>> {
    let instance: RetreatGalleriesModel = RetreatGalleriesEntity::find()
        .filter(RetreatGalleriesColumn::GalleryId.eq(gallery_id))
        .filter(RetreatGalleriesColumn::RetreatId.eq(retreat_id))
        .one(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            to_error_response_with_message("Retreat gallery not found.", StatusCode::NOT_FOUND)
        })?;

    // todo check if the logged in user is from the retreat

    // Convert to ActiveModel for editing
    let active_model: RetreatGalleriesActiveModel = instance.into_active_model();

    active_model
        .delete(&state.database)
        .await
        .map_err(|e| to_error_response(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    // Convert model to serializer
    Ok(CustomResponse::builder({})
        .message("Retreat gallery deleted successfully.")
        .status_code(StatusCode::NO_CONTENT)
        .build())
}

pub fn retreat_gallery_router() -> Router<AppState> {
    let router = Router::new()
        .route(
            "/retreats/{retreat_id}/galleries/",
            post(create_retreat_gallery),
        )
        .route(
            "/retreats/{retreat_id}/galleries/",
            get(list_retreat_gallery),
        )
        .route(
            "/retreats/{retreat_id}/galleries/{review_id}/",
            patch(update_retreat_gallery),
        )
        .route(
            "/retreats/{retreat_id}/galleries/{review_id}/",
            delete(delete_retreat_gallery),
        );
    return router;
}
