use json::{object, JsonValue};
use std::sync::mpsc::Sender;
use tungstenite::{connect, Message};
use url::Url;

pub fn kraken_tracker(tx: &Sender<(&'static str, f64)>) {
    // get transmitting end
    let kraken_tx = tx.clone();

    // new tokio task
    tokio::task::spawn_blocking(move || {
        // if we can't open the socker for whatever reason we just abort
        let (mut socket, _) =
            connect(Url::parse("wss://ws.kraken.com").unwrap()).expect("Can't connect to Kraken");

        // message to subscribe to xbt / usd ticker
        let subscribe_message = object! {
            event: "subscribe",
            pair: [ "XBT/USD" ],
            subscription: object!{
                name: "ticker"
            }
        };

        // send subscribe_message
        socket
            .write_message(Message::Text(subscribe_message.dump()))
            .unwrap();
        loop {
            // we crash application on connection closed
            if let Message::Text(msg) = socket.read_message().expect("Kraken closed the connection")
            {
                // parse message
                let msg = json::parse(&msg).unwrap();

                // close (c) price value
                if let JsonValue::Short(num) = msg[1]["c"][0].clone() {
                    // send value and id
                    kraken_tx
                        .send(("Kraken", num.parse::<f64>().unwrap()))
                        .unwrap();
                }
            }
        }
    });
}
