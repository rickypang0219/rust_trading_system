mod bookticker_stream;
use bookticker_stream::bookticker::BookTickerStream;
mod management;
mod shared;
use shared::aws_client::get_binance_api_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bookticker_stream = BookTickerStream::new();
    // let api_key: Option<String> = get_binance_api_key().await?;
    // println!("{:?}", api_key);
    let listener_task = {
        let bookticker_stream_clone = bookticker_stream.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = bookticker_stream_clone.listen_coins_book_prices().await {
                    eprintln!("Error listening to WebSocket: {:?}", e);
                    continue;
                }
            }
        })
    };

    let printer_task = {
        let bookticker_stream_clone = bookticker_stream.clone();
        tokio::spawn(async move {
            loop {
                bookticker_stream_clone.show_bookticker().await;
            }
        })
    };
    let _ = tokio::try_join!(listener_task, printer_task);
    Ok(())
}
