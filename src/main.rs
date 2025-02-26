mod models;
mod routes;
mod handlers;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum_server::Server;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
};
use sea_query::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {    
    let app = Router::new()
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_routes::user_routes());
    

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
