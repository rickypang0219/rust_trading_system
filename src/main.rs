use axum::{extract::Query, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::join;

mod websockets;
use websockets::price_data::listen_coins_limit_prices;

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

#[tokio::main]
async fn main() {
    let app = Router::new().route("/order", post(handle_signal));

    // Define the address to listen on
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
            listen_coins_limit_prices().await;
        }
    );
}
