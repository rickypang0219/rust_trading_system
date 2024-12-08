use crate::binance_client::client::BinanceClient;
use reqwest;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
const USER_DATA_WS: &str = "wss://fstream.binance.com/ws/";
const KEEP_ALIVE_INTERVAL: u64 = 1800;

pub struct OrderUpdateStream {
    client: BinanceClient,
}

impl OrderUpdateStream {
    pub fn new(client: BinanceClient) -> Self {
        OrderUpdateStream { client }
    }

    async fn maintain_long_live_listen_key(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval =
            tokio::time::interval(tokio::time::Duration::from_secs(KEEP_ALIVE_INTERVAL));
        loop {
            interval.tick().await;
            if let Err(e) = self.send_keep_alive_message().await {
                eprintln!("Failed to keep listen key alive: {}", e);
            }
        }
    }

    async fn send_keep_alive_message(&self) -> Result<(), reqwest::Error> {
        Ok(())
    }

    pub async fn listen_order_updates(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        Ok(())
    }
}
