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

const PORT: u16 = 5555;

#[tokio::main]
async fn main() {
    let corpus_json = fs::read_to_string("corpus.json").expect("Should be able to read file");
    let text_table = TextTable::from_json(&corpus_json);

    let app = Router::new()
        .route("/", get(make_shore))
        .with_state(text_table)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.expect(&format!("Can bind to port {PORT}"));
    axum::serve(listener, app).await.expect("Should be able to start server");
}

async fn make_shore(State(text_table): State<TextTable>) -> Json<Shore> {
    let shore = Shore::new(20, 200, text_table);
    println!("{shore}");
    Json(shore)
}