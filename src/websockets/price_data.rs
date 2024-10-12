use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize, Deserialize, Debug)]
struct BookTicker {
    #[serde(rename = "e")]
    event: String,
    #[serde(rename = "u")]
    update_id: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    best_bid: String,
    #[serde(rename = "B")]
    bbid_qty: String,
    #[serde(rename = "a")]
    best_ask: String,
    #[serde(rename = "A")]
    ask_qty: String,
    #[serde(rename = "T")]
    trans_time: u64,
    #[serde(rename = "E")]
    event_time: u64,
}

pub async fn fetch_limit_prices(symbol: &str) -> Result<(), Box<dyn std::error::Error + Send>> {
    let url: String = format!(
        "wss://fstream.binance.com/ws/{}@bookTicker",
        symbol.to_lowercase()
    );
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect!");
    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let ticker: BookTicker =
                    serde_json::from_str(&text).expect("JSON was not well format!");
                println!(
                    "{:?}, {:?} {:?}",
                    ticker.symbol,
                    ticker
                        .best_bid
                        .parse::<f64>()
                        .expect("Non convertible to float"),
                    ticker
                        .best_ask
                        .parse::<f64>()
                        .expect("Non convertible to float"),
                );
            }
            _ => {
                println!("Received Non Text Messages!")
            }
        }
    }
    Ok(())
}

pub async fn listen_coins_limit_prices() {
    let coins: [&str; 3] = ["BTCUSDT", "ETHUSDT", "ZENUSDT"];
    let mut handles = vec![];
    for &symbol in &coins {
        let handle = tokio::spawn(fetch_limit_prices(symbol));
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Error in task: {:?}", e);
        }
    }
}
