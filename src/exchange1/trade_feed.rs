// use std::thread::sleep;
// use std::time::Duration;

use tokio::sync::mpsc::Receiver;
use tracing::{info,error};
use crate::ftypes::Trade;

pub struct TradeFeed {

    pub rx: Receiver<Trade>,
}

impl TradeFeed {
    pub fn new(rx: Receiver<Trade>)-> Self {
        TradeFeed {
            rx,
        }
    }

    pub async fn start(&mut self) {
        loop {
            // sleep(Duration::from_millis(5000));
            
            
            let res = self.rx.recv().await;
            match res {
                Some(t) => {
                    // info!("Trade sent");
                    info!("RECV On Trade channel {}", t.as_log());


                    // logic to send to exchange
                }
                None => {
                    error!("No Trade yet");
                }
            }

            
        }
    }
}