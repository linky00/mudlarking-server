use crate::{font::*, shore::*, text_table::*};
use axum::{extract::State, routing::get, Json, Router};
use dotenv::dotenv;
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

mod font;
mod shore;
mod text_table;

const SHORE_WIDTH: f32 = 800.0;
const SHORE_HEIGHT: f32 = 1200.0;
const FONT_SIZE: f32 = 16.0;

struct AppState<'a> {
    font_sizer: FontSizer<'a>,
    text_table: TextTable,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let font_sizer = font::FontSizer::new(include_bytes!("EBGaramond-Regular.ttf"), 0, FONT_SIZE);

    let corpus_json = include_str!("corpus.json");
    let text_table = TextTable::from_json(corpus_json);

    let app_state = AppState {
        font_sizer,
        text_table,
    };

    let port = env::var("PORT").expect("'PORT' env should be set");

    let app = Router::new()
        .route("/", get(get_shore))
        .route("/debug", get(get_debug_shore))
        .with_state(Arc::new(app_state))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap_or_else(|_| panic!("Cannot bind to port {port}"));
    println!("Starting server on port {port}...");
    axum::serve(listener, app)
        .await
        .expect("Should be able to start server");
}

async fn get_shore(State(app_state): State<Arc<AppState<'_>>>) -> Json<Shore> {
    let shore = Shore::new(
        SHORE_WIDTH,
        SHORE_HEIGHT,
        &app_state.text_table,
        &app_state.font_sizer,
    );
    Json(shore)
}

async fn get_debug_shore(State(app_state): State<Arc<AppState<'_>>>) -> Json<DebugShore> {
    let shore = DebugShore::new(
        SHORE_WIDTH,
        SHORE_HEIGHT,
        &app_state.text_table,
        &app_state.font_sizer,
    );
    Json(shore)
}
