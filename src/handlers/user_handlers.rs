use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use serde_json::json;
use uuid::Uuid;
use crate::models::user_models::{UpdateUserModel, UserModel};

pub async fn update_user_put(
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:123456@localhost:5432/BlogDB")
            .await
            .unwrap();

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap().unwrap().into();

    user.name = Set(user_data.name);
    user.update(&db).await.unwrap();

    db.close().await.unwrap();

    (StatusCode::OK, Json(json!({"message": "data updated"})))
}

pub async fn delete_user_delete(
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:123456@localhost:5432/BlogDB")
            .await
            .unwrap();

    let user = entity::user::Entity::find().filter(entity::user::Column::Uuid.eq(uuid)).one(&db).await.unwrap().unwrap();
    entity::user::Entity::delete_by_id(user.id).exec(&db).await.unwrap();

    db.close().await.unwrap();

    (StatusCode::OK, Json(json!({"message": "user deleted"})))
}

pub async fn fetch_all_get() -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:123456@localhost:5432/BlogDB")
            .await
            .unwrap();

    let users: Vec<UserModel> = entity::user::Entity::find().all(&db).await.unwrap().into_iter().map(|item| UserModel{
        name: item.name,
        email: item.email,
        password: item.password,
        uuid: item.uuid,
        created_at: item.created_at,
    }).collect();

    db.close().await.unwrap();

    (StatusCode::OK, Json(users))
}