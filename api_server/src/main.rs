mod error;
mod image_analyzer;
mod pixel_data;
mod rgb_ext;
mod web_schema;

use std::net::SocketAddr;
use std::{collections::HashMap, io::Read};

use error::Error;
use image::{guess_format, Rgb};
use image_analyzer::ImageAnalyzer;
use pixel_data::PixelData;
use web_schema::{CategorizedImage, ImageSelectionFilter};

use axum::{
    body::Body,
    extract::{ConnectInfo, Extension, Json, Path, Query},
    http::{Response, StatusCode},
    routing::{get, post},
    AddExtensionLayer, Router,
};
use http::Method;
use rust_decimal::Decimal;
use sqlx::{mysql::MySqlPool, Executor, Row};
use tower_http::cors::{any, CorsLayer};

use tracing::{info, instrument, span, Level};
use tracing_subscriber::{prelude::*, Layer};

use std::sync::Arc;

use crate::rgb_ext::HexCode;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::EXIT)
        .init();

    let _ = span!(Level::INFO, "server_span").entered();

    let db = Arc::new(MySqlPool::connect("mysql://root:pass1234@db/core").await?);

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(any());

    let app = Router::new()
        .route("/", get(index))
        .route("/get_star", get(get_star))
        .route("/add_star", get(add_star))
        .route("/upload_image", post(upload_image))
        .route("/image/:image_path", get(get_image))
        .route("/recommendation", post(get_recommendation))
        
        .layer(cors)
        .layer(AddExtensionLayer::new(db));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8089));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
        .await
        .unwrap();

    Ok(())
}

#[instrument]
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>, db: Extension<Arc<MySqlPool>>,) -> Result<Json<Vec<(String, String, String)>>, StatusCode> {
    let ret = sqlx::query("SELECT * FROM images LIMIT 100")
    .fetch_all(&*db.0)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let ret: Vec<_> = ret.into_iter()
        .map(|row| (row.get::<String, usize>(0), row.get::<String, usize>(1), row.get::<String, usize>(2)))
        .collect();
    
    Ok(Json(ret))
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
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[instrument]
async fn get_recommendation(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
    Json(filter): Json<ImageSelectionFilter>,
) -> Result<Json<Vec<(String, String)>>, StatusCode> {
    let colors: Vec<_> = filter
        .colors
        .into_iter()
        .map(|e| Rgb::from(HexCode::new(e)))
        .map(|color| {
            (
                PixelData::get_area_index(color[0], 4),
                PixelData::get_area_index(color[1], 4),
                PixelData::get_area_index(color[2], 4),
            )
        })
        .map(|idx| PixelData::index_to_string(idx))
        .collect();

    let mut color_query = String::new();

    for color in colors {
        color_query.push_str("%|");
        color_query.push_str(&color);
    }
    color_query.push('%');

    let mut food_query = String::new();

    for food in filter.foods {
        food_query.push_str(&food);
        food_query.push(',');
    }
    food_query.push_str("NULL");

    let result = sqlx::query(
        "SELECT (path, category) from images WHERE data LIKE ? AND category NOT IN (?)",
    )
    .bind(color_query)
    .bind(food_query)
    .fetch_all(&*db.0)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<_> = result
        .into_iter()
        .take(5)
        .map(|row| (row.get::<String, usize>(0), row.get::<String, usize>(1)))
        .collect();

    Ok(Json(result))
}

//#[instrument(skip(payload))]
async fn upload_image(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
    Json(payload): Json<CategorizedImage>,
) -> Result<String, StatusCode> {
    let image_buffer =
        base64::decode(&payload.image_buffer).map_err(|_| StatusCode::BAD_REQUEST)?;

    let analyzer = ImageAnalyzer::new(&image_buffer, &payload.image_type)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let analyzed_data = analyzer.pixel_data().await.into_string(6).await;
    let path = analyzer
        .save_with_format(image::ImageFormat::Png)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let category = payload.category;

    sqlx::query("INSERT INTO images VALUES (?, ?, ?)")
        .bind(path.clone())
        .bind(category)
        .bind(analyzed_data)
        .execute(&*db.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(path)
}

#[instrument]
async fn get_image(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<Arc<MySqlPool>>,
    Path(image_path): Path<String>,
) -> Result<Response<Body>, StatusCode> {
    let mut img = std::fs::File::open(format!("./data/images/{}", image_path))
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut img_bytes = Vec::new();

    img.read_to_end(&mut img_bytes)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let format = guess_format(&img_bytes)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .extensions_str()[0];
    let format = format!("image/{}", format);

    Ok(Response::builder()
        .header("Content-Type", format)
        .body(img_bytes.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}
