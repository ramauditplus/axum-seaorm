use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use sea_query::*;
use serde_json::json;
use uuid::Uuid;
use entity::user;
use crate::models::user_models::{CreateUserModel, LoginUserModel, UserModel};

pub async fn create_user_post(Json(user_data): Json<CreateUserModel>) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:123456@localhost:5432/BlogDB")
            .await
            .unwrap();

    let user_model = user::ActiveModel {
        id: Default::default(),
        name: Set(user_data.name),
        email: Set(user_data.email),
        password: Set(user_data.password),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().date_naive()),
        ..Default::default()
    };

    user_model.insert(&db).await.unwrap();

    db.close().await.unwrap();

    (
        StatusCode::OK,
        Json(json!({"message": "data inserted into user table"})),
    )
}

pub async fn login_user_post(Json(user_data): Json<LoginUserModel>) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:123456@localhost:5432/BlogDB")
            .await
            .unwrap();

    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(user_data.email))
                .add(entity::user::Column::Password.eq(user_data.password)),
        )
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let data: UserModel = UserModel {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: user.uuid,
        created_at: user.created_at,
    };

    db.close().await.unwrap();

    (StatusCode::OK, Json(data))
}