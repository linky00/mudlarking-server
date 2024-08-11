use crate::font::*;
use crate::shore::*;
use crate::text_table::*;
use axum::{extract::State, routing::get, Json, Router};
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

mod font;
mod shore;
mod text_table;

struct AppState<'a> {
    font_sizer: FontSizer<'a>,
    text_table: TextTable,
}

#[tokio::main]
async fn main() {
    let font_sizer = font::FontSizer::new(include_bytes!("EBGaramond-Regular.ttf"), 0, 16.0);

    let corpus_json = include_str!("corpus.json");
    let text_table = TextTable::from_json(corpus_json);

    let app_state = AppState {
        font_sizer,
        text_table,
    };

    let port = env::var("PORT").expect("'PORT' env should be set");

    let app = Router::new()
        .route("/", get(make_shore))
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

async fn make_shore(State(app_state): State<Arc<AppState<'_>>>) -> Json<Shore> {
    let shore = Shore::new(800.0, 1200.0, &app_state.text_table, &app_state.font_sizer);
    Json(shore)
}
