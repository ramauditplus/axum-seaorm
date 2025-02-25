mod models;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use axum_server::Server;
use chrono::Utc;
use entity::user;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
};
use sea_query::*;
use serde_json::json;
use std::net::SocketAddr;
use uuid::Uuid;
// use sea_query::JoinOn::Condition;
use crate::models::user_models::{CreateUserModel, LoginUserModel, UpdateUserModel, UserModel};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/user/register", post(create_user_post))
        .route("/api/user/login", post(login_user_post))
        .route("/api/user/{uuid}/update", put(update_user_put))
        .route("/api/user/{uuid}/delete", put(delete_user_delete))
        .route("/api/user/all", get(fetch_all_get));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_route() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Hello, World!"})))
}

async fn create_user_post(Json(user_data): Json<CreateUserModel>) -> impl IntoResponse {
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

async fn login_user_post(Json(user_data): Json<LoginUserModel>) -> impl IntoResponse {
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

async fn update_user_put(
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

async fn delete_user_delete(
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

async fn fetch_all_get() -> impl IntoResponse {
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