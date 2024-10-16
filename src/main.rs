use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::join;
mod websockets;
use websockets::price_data::listen_coins_book_prices;
use websockets::structs::BestPrices;

mod execution;
mod management;
mod shared;

#[derive(Deserialize, Debug)]
struct SignalQuery {
    symbol: String,
    timestamp: u64,
    strategy: String,
    bet_size: f32,
    target_position: f32,
}

async fn handle_signal(Query(order): Query<SignalQuery>) {
    println!(
        "Received order:\nSymbol: {}\nTimestamp: {}\nStrategy: {}\nBet Size: {}\nTarget Position: {}",
        order.symbol, order.timestamp, order.strategy, order.bet_size, order.target_position
    );
}

async fn get_book_ticker(
    State(book_ticker): State<Arc<tokio::sync::Mutex<HashMap<String, BestPrices>>>>,
) -> impl IntoResponse {
    let book_ticker = book_ticker.lock().await;

    println!("GET /bookTicker - Status: {}", StatusCode::OK);

    (StatusCode::OK, Json(book_ticker.clone()))
}

#[tokio::main]
async fn main() {
    let book_ticker: Arc<tokio::sync::Mutex<HashMap<String, BestPrices>>> =
        Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/bookTicker", get(get_book_ticker))
        .route("/order", post(handle_signal))
        .with_state(book_ticker.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{}", addr);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    join!(
        async {
            if let Err(e) = server.await {
                eprintln!("Server error: {:?}", e);
            }
        },
        async {
            if let Err(e) = listen_coins_book_prices(book_ticker).await {
                eprintln!("WebSocket error: {:?}", e);
            }
        }
    );
}
