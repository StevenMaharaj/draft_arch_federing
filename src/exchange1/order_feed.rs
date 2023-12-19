use std::thread::sleep;
use std::time::Duration;

use tokio::sync::mpsc::Sender;
use tracing::{info,error};
use crate::ftypes::{Order,Side};

pub struct OrderFeed {

    pub tx: Sender<Order>,
}

impl OrderFeed {
    pub fn new(tx: Sender<Order>)-> Self {
        OrderFeed {
            tx,
        }
    }

    pub async fn start(&self) {
        loop {
            sleep(Duration::from_millis(1000));
            let o= Order::new(12.3,10.9,Side::Buy,"BTCUSDT".to_string());
            
            
            let res = self.tx.send(o.clone()).await;
            println!("trade ch {}",self.tx.capacity());
            match res {
                Ok(_) => {
                    // info!("Order sent");
                    info!("SENT On Order channel {}", o.as_log());
                }
                Err(e) => {
                    error!("Error sending order: {} for order {}", e, o.as_log());
                }
            }

            
        }
    }
}