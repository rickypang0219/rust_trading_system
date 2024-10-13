use super::structs::{BestPrices, BookTicker};
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn listen_coins_book_prices(
    book_ticker: Arc<tokio::sync::Mutex<HashMap<String, BestPrices>>>,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    let url: String = format!("wss://fstream.binance.com/ws/!bookTicker",);
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect!");
    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let ticker: BookTicker =
                    serde_json::from_str(&text).expect("JSON was not well format!");

                let bid: f64 = ticker
                    .best_bid
                    .parse::<f64>()
                    .expect("Failed to parse as f64");
                let ask: f64 = ticker
                    .best_ask
                    .parse::<f64>()
                    .expect("Failed to parse as f64");

                {
                    let mut book_ticker = book_ticker.lock().await;
                    book_ticker.insert(ticker.symbol.clone(), BestPrices { bid, ask });
                }
            }
            _ => {
                println!("Received Non Text Messages!")
            }
        }
    }
    Ok(())
}
