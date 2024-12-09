use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BestPrices {
    pub bid: f64,
    pub ask: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BookTicker {
    #[serde(rename = "e")]
    pub event: String,
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid: String,
    #[serde(rename = "B")]
    pub bbid_qty: String,
    #[serde(rename = "a")]
    pub best_ask: String,
    #[serde(rename = "A")]
    pub ask_qty: String,
    #[serde(rename = "T")]
    pub trans_time: u64,
    #[serde(rename = "E")]
    pub event_time: u64,
}

#[derive(Debug, Clone)]
pub struct BookTickerStream {
    pub book_ticker: Arc<tokio::sync::Mutex<HashMap<String, BestPrices>>>,
}

impl Default for BookTickerStream {
    fn default() -> Self {
        Self::new()
    }
}

impl BookTickerStream {
    pub fn new() -> Self {
        BookTickerStream {
            book_ticker: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    pub async fn listen_coins_book_prices(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        loop {
            let url: String = "wss://fstream.binance.com/ws/!bookTicker".to_string();
            let (ws_stream, _) = match connect_async(&url).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to connect: {}, retrying...", e);
                    continue; // Retry immediately without delay
                }
            };
            let (mut write, mut read) = ws_stream.split();
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
                            let mut book_ticker = self.book_ticker.lock().await;
                            book_ticker.insert(ticker.symbol.clone(), BestPrices { bid, ask });
                        }
                        println!(" Received Book Ticker {:?}", ticker);
                    }
                    Ok(Message::Ping(payload)) => {
                        if let Err(e) = write.send(Message::Pong(payload)).await {
                            eprintln!("Failed to send Pong response: {}", e);
                        }
                    }
                    Ok(non_text_message) => {
                        println!("Received Non Text Messages {:?}", non_text_message)
                    }
                    Err(e) => {
                        eprintln!("Error Message {}", e);
                        break;
                    }
                }
            }
            eprintln!("Connection lost, retrying immediately...");
        }
    }

    pub async fn show_bookticker(&self) {
        loop {
            time::sleep(time::Duration::new(1800, 0)).await;
            let book_ticker = self.book_ticker.lock().await;
            println!("Current Book Ticker:");
            for (symbol, prices) in book_ticker.iter() {
                println!("{}: Bid: {}, Ask: {}", symbol, prices.bid, prices.ask);
            }
        }
    }
}
