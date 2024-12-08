use hex;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    symbol: String,
    side: String,
    #[serde(rename = "type")]
    order_type: String,
    quantity: f64,
    signature: String,
}

#[derive(Debug)]
pub struct BinanceClient {
    api_key: String,
    secret_key: String,
    client: Client,
}

const BASE_URL: &str = "https://fapi.binance.com/fapi/v1/";
const LISTEN_KEY_ENDPOINT: &str = "/fapi/v1/listenKey";

impl BinanceClient {
    pub fn new(api_key: String, secret_key: String) -> Self {
        BinanceClient {
            api_key,
            secret_key,
            client: Client::new(),
        }
    }

    fn generate_hmac_key(&self) -> Result<HmacSha256, Box<dyn std::error::Error>> {
        if !self.secret_key.is_empty() {
            let mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())?;
            Ok(mac)
        } else {
            Err("The Secret Key is not given".into())
        }
    }

    fn generate_signature(&self, params: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut hmac_key = self.generate_hmac_key()?;
        hmac_key.update(params.as_bytes());
        let signature = hex::encode(hmac_key.finalize().into_bytes());
        Ok(signature)
    }

    async fn get_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
        let result: reqwest::Response = self.client.get(url).send().await?;
        let data: serde_json::Value = result.json::<serde_json::Value>().await?;
        Ok(data)
    }

    async fn post_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
        let result: reqwest::Response = self.client.post(url).send().await?;
        let data: serde_json::Value = result.json().await?;
        Ok(data)
    }

    async fn put_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
        let result: reqwest::Response = self.client.put(url).send().await?;
        let data: serde_json::Value = result.json().await?;
        Ok(data)
    }

    pub async fn post_request_with_sign<I>(
        &self,
        url: String,
        params_with_sign: I,
    ) -> Result<serde_json::Value, reqwest::Error>
    where
        I: Serialize,
    {
        let result: reqwest::Response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .json(&params_with_sign)
            .send()
            .await?;

        let data: serde_json::Value = result.json::<serde_json::Value>().await?;
        Ok(data)
    }

    pub async fn create_futures_order(
        &self,
        order: &Order,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", BASE_URL, "/order");
        let params = format!(
            "symbol={}&side={}&type={}&quantity={}",
            order.symbol, order.side, order.order_type, order.quantity
        );

        let signature = self.generate_signature(params)?;

        let order_with_signature = Order {
            signature,
            ..order.clone()
        };
        let order_response = self
            .post_request_with_sign(url, order_with_signature)
            .await?;
        Ok(order_response)
    }

    pub async fn get_futures_order(
        &self,
        _order_id: &str,
        _symbol: &str,
    ) -> Result<(), reqwest::Error> {
        Ok(())
    }

    pub async fn cancel_futures_order(
        &self,
        _order_id: &str,
        _symbol: &str,
    ) -> Result<(), reqwest::Error> {
        Ok(())
    }

    pub async fn get_futures_exchange_info(&self) -> Result<serde_json::Value, reqwest::Error> {
        let url: String = format!("{}{}", BASE_URL, "exchangeInfo");
        let exchange_info: Result<serde_json::Value, reqwest::Error> = self.get_request(url).await;
        match exchange_info {
            Ok(respons) => Ok(respons),
            Err(e) => {
                eprintln!("Failed to obtain exchange info {}", e);
                Err(e)
            }
        }
    }
}
