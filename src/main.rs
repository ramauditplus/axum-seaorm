use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router, serve};
use axum_server::Server;
use chrono::{NaiveDate, Utc};
use entity::user;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use serde_json::json;
use std::net::SocketAddr;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/test", get(test_route)).route("/api/test/insert", post(create_user));
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

async fn create_user() -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("postgres://postgres:1@localhost:5432/BlogDB")
        .await
        .unwrap();

    let user_model = user::ActiveModel {
        id: Default::default(),
        name: Set("test".to_owned()),
        email: Set("test@gmail.com".to_owned()),
        password: Set("123456".to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(NaiveDate::from(Utc::now().naive_utc())),
    };

    let out = user_model.insert(&db).await.unwrap();

    (StatusCode::OK, Json(json!({"message": "data inserted into user table"})))
}
