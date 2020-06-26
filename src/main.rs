pub mod bitstamp_tracker;
pub mod hitbtc_tracker;
pub mod kraken_tracker;

use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::HashMap;

#[tokio::main]
async fn main() {

    // channel for data (ExchangeId, prince)
    let (tx, rx): (Sender<(&str,f64)>, Receiver<(&str,f64)>) = mpsc::channel();

    // start trackers (this spawns tokio tasks)
    crate::kraken_tracker::kraken_tracker(&tx);
    crate::hitbtc_tracker::hitbtc_tracker(&tx);
    crate::bitstamp_tracker::bitstamp_tracker(&tx);

    // storage for data (extendable to more exchanges)
    let mut prices = HashMap::new();

    loop {
        // blocks main thread until a message is recieved
        let (id, price) = rx.recv().unwrap();
        prices.insert(id, price);

        // clears screen on linux and windows and puts cursor top left (to get clean output)
        print!("\x1B[2J\x1B[H");

        // print prices between 2 lines made out of =
        // it is assumed that console is >= 80 char
        let line = "=".repeat(80);
        println!("{}", line);
        for (&id, price) in prices.iter() {
            println!("{}: ${}", id, price);
        }
        println!("{}", line);
    }

    /*
    we could return tokio tasks and await for them here
    but we already have infinite loop above so the main
    thread never ends.
    
    kraken_task.await.unwrap();
    hitbtc_task.await.unwrap();
    bitstamp_task.await.unwrap();
    */

}
