use json::{object, JsonValue};
use std::sync::mpsc::Sender;
use tungstenite::{connect, Message};
use url::Url;

pub fn hitbtc_tracker(tx: &Sender<(&'static str, f64)>) {
    // get transmitting end
    let hitbtc_tx = tx.clone();

    // new tokio task
    tokio::task::spawn_blocking(move || {
        // if we can't open the socker for whatever reason we just abort
        let (mut socket, _) = connect(Url::parse("wss://api.hitbtc.com/api/2/ws").unwrap())
            .expect("Can't connect to hitbtc");

        // subscribe message (only 1 trade from history for current price)
        let subscribe_message = object! {
            method: "subscribeTrades",
            params: object!{
                symbol: "BTCUSD",
                limit: 1
            }
        };

        // subscribe to trades
        socket
            .write_message(Message::Text(subscribe_message.dump()))
            .unwrap();
        loop {
            // we abort on conection closed
            if let Message::Text(msg) = socket.read_message().expect("Hitbtc closed connection") {
                // parse message
                let msg = json::parse(&msg).unwrap();

                // deal only with messages that are connected to trades
                if (msg["method"] == "updateTrades") | (msg["method"] == "snapshotTrades") {
                    if let JsonValue::Short(num) = msg["params"]["data"][0]["price"].clone() {
                        // send data
                        hitbtc_tx
                            .send(("HitBTC", num.parse::<f64>().unwrap()))
                            .unwrap();
                    }
                }
            }
        }
    });
}
