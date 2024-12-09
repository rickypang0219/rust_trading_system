mod bookticker_stream;
use bookticker_stream::bookticker::BookTickerStream;
mod management;
mod shared;

#[tokio::main]
async fn main() {
    let bookticker_stream = BookTickerStream::new();
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
}
