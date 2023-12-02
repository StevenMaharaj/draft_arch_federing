use std::thread::sleep;
use std::time::Duration;

use tokio::sync::oneshot::Sender;
use tracing::info;
use crate::ftypes::Order;

pub struct OrderFeed {

    pub tx: Sender<Order>,
}

impl OrderFeed {
    pub fn new(tx: Sender<Order>)-> Self {
        OrderFeed {
            tx,
        }
    }

    pub fn start(&self) {
        loop {
            sleep(Duration::from_millis(5000));
            let o= Order::new(12.3,10.9);
            info!("{}", o.as_log());
            self.tx.send(o);

            
        }
    }
}