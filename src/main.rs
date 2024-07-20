use axum::{
    extract::State,
    routing::get,
    Json,
    Router,
};
use std::fs;
use tower_http::cors::CorsLayer;
use crate::shore::*;
use crate::text_table::*;

mod shore;
mod text_table;

#[tokio::main]
async fn main() {
    let corpus_json = fs::read_to_string("corpus.json").expect("Can read file");
    let text_table = TextTable::from_json(&corpus_json);

    let app = Router::new()
        .route("/", get(make_shore))
        .with_state(text_table)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn make_shore(State(text_table): State<TextTable>) -> Json<Shore> {
    let shore = Shore::new(20, 200, text_table);
    println!("{shore}");
    Json(shore)
}