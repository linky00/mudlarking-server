use axum::{
    extract::State,
    routing::get,
    Json,
    Router,
};
use std::env;
use tower_http::cors::CorsLayer;
use crate::shore::*;
use crate::text_table::*;

mod shore;
mod text_table;

#[tokio::main]
async fn main() {
    let corpus_json = include_str!("corpus.json");
    let text_table = TextTable::from_json(&corpus_json);

    let port = env::var("PORT").expect("'PORT' env should be set");

    let app = Router::new()
        .route("/", get(make_shore))
        .with_state(text_table)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.expect(&format!("Can bind to port {port}"));
    axum::serve(listener, app).await.expect("Should be able to start server");
}

async fn make_shore(State(text_table): State<TextTable>) -> Json<Shore> {
    let shore = Shore::new(25, 200, text_table);
    println!("{shore}");
    Json(shore)
}