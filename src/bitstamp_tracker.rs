use json::{object, JsonValue};
use std::sync::mpsc::Sender;
use tungstenite::{connect, Message};
use url::Url;

pub fn bitstamp_tracker(tx: &Sender<(&'static str, f64)>) {
    // get transmitting end
    let bitstamp_tx = tx.clone();

    // spawn new task
    tokio::task::spawn_blocking(move || {
        // connect to bitstamp (or abort if can't)
        let (mut socket, _) = connect(Url::parse("wss://ws.bitstamp.net").unwrap())
            .expect("Can't connect to Bitstamp");

        // subscribe message
        let subscribe_message = object! {
            event: "bts:subscribe",
            data: object!{
                channel: "live_trades_btcusd"
            }
        };

        // subscribe to trades
        socket
            .write_message(Message::Text(subscribe_message.dump()))
            .unwrap();
        loop {
            // read messages
            if let Message::Text(msg) = socket.read_message().expect("Bitstamp closed connection") {
                // parse message
                let msg = json::parse(&msg).unwrap();

                // only concerned with trades
                if msg["event"] == "trade" {
                    if let JsonValue::Number(num) = msg["data"]["price"].clone() {
                        let num: f64 = num.into();
                        // send message
                        bitstamp_tx.send(("Bitstamp", num)).unwrap();
                    }
                }
            }
        }
    });
}
