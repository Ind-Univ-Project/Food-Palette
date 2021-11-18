mod web_schema;
mod error;
mod image_analyzer;
mod pixel_data;
mod rgb_ext;

use std::collections::HashMap;
use std::net::SocketAddr;

use web_schema::CategorizedImage;
use error::Error;
use image_analyzer::ImageAnalyzer;

use axum::{
    extract::{ConnectInfo, Extension, Json, Query},
    http::StatusCode,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use rust_decimal::Decimal;
use sqlx::{mysql::MySqlPool, Executor, Row};

use tracing::{info, instrument, span, Level};
use tracing_subscriber::{prelude::*, Layer};

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();

    let _ = span!(Level::INFO, "server_span").entered();

    let db = Arc::new(MySqlPool::connect("mysql://root:pass1234@db/core").await?);

    let app = Router::new()
        .route("/", get(index))
        .route("/get_star", get(get_star))
        .route("/add_star", get(add_star))
        .route("upload_image", post(upload_image))
        .layer(AddExtensionLayer::new(db));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8089));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
        .await
        .unwrap();

    Ok(())
}

#[instrument]
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> &'static str {
    "Welcome to API Server"
}

#[instrument]
async fn get_star(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
) -> Result<String, StatusCode> {
    let point = sqlx::query("SELECT AVG(point) FROM stars")
        .fetch_one(&*db.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let point = point.get::<Decimal, usize>(0).to_string();

    Ok(point)
}

#[instrument]
async fn add_star(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<StatusCode, StatusCode> {
    let star = params.get("star").ok_or(StatusCode::BAD_REQUEST)?;

    sqlx::query("INSERT INTO stars VALUES (NULL, ?)")
        .bind(star)
        .execute(&*db.0)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(StatusCode::OK)
}

#[instrument]
async fn get_reccommendation(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
) {
    //get json from request
    //filtering with user_dislike, user_recents, color_data -> with db query
    //response
}

#[instrument(skip(payload))]
async fn upload_image(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
    Json(payload): Json<CategorizedImage>,
) -> Result<String, StatusCode> {
    let analyzer = ImageAnalyzer::new(payload.image_buffer.as_bytes(), &payload.image_type)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let analyzed_data = analyzer.pixel_data().await.into_string(6).await;
    let path = analyzer
        .save_with_format(image::ImageFormat::Png)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let category = payload.category;

    sqlx::query("INSERT INTO images (path, category, data) VALUES (?, ?, ?)")
        .bind(&path)
        .bind(category)
        .bind(analyzed_data)
        .execute(&*db.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(path)
}

#[instrument]
async fn get_image() {
    //parse get method parameter
    //response image file
}
