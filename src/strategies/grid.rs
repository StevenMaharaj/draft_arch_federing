use super::Strategy;
use std::collections::HashMap;
use serde::Deserialize;
use tracing::info;
use crate::{ftypes::{Order, Trade,Side}, exchange1::trade_feed::TradeFeed};
use tokio::sync::mpsc::{Receiver,Sender, self};
use crate::exchange1::order_feed::OrderFeed;
use async_trait::async_trait;


const STRATEGY_NAME: &str = "grid";
const EXCHANGE: &str = "EX1";


#[derive(Debug, Deserialize)]
struct GridParam {
    amount: f32,
    spread: f32,
    symbol: String

}


pub(crate) struct GridStrategy {
    exchange: String,
    symbols: Vec<String>,
    grid_params: HashMap<String,GridParam>,
    orders: HashMap<String, i32>,
    order_rx: Receiver<Order>,
    trade_tx: Sender<Trade>,


}
#[async_trait]
impl Strategy for GridStrategy {
    
    fn new() -> Self {
        let exchange = EXCHANGE.to_string();
        
        let grid_params =  Self::load_grid_params();
        let symbols = grid_params.keys().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut orders = HashMap::new();
        symbols.iter().for_each(|s| {
            orders.insert(s.to_string(),0);
        });

        // set up channels

        let (order_tx, order_rx) = mpsc::channel::<Order>(100);
        let (trade_tx, trade_rx) = mpsc::channel::<Trade>(100);


        tokio::spawn(async move {
            println!("Order feed thread spawned");
            let order_feed = OrderFeed::new(order_tx);
            order_feed.start().await;
        });

        tokio::spawn(async move {
            println!("Trade feed thread spawned");
            let mut trade_feed = TradeFeed::new(trade_rx);
            trade_feed.start().await;
        });

        Self {exchange,symbols,grid_params,orders ,order_rx, trade_tx}
        
    }


    async fn on_tick(&mut self) {
        // todo!()
        println!("on tick");
        let new_order = self.order_rx.recv().await;
        match new_order {
            Some(o) => {
                println!("Got order {}", o.as_log());
                let trade = self.replace_order(&o);
                println!("Sending trade {}", trade.as_log());
                let res = self.trade_tx.send(trade.clone()).await;
                println!("Trade chanel grid {}",self.trade_tx.capacity());
                match res {
                    Ok(_) => {
                        info!("SENT On Trade channel {}", trade.as_log())
                    }
                    Err(e) => {
                        println!("Error sending trade: {}", e);
                    }
                }
            }
            None => {
                println!("No order received");
            }
        }
        // look at order channel
        // Check we are with the limits
        // 
        // if buy, replace with a sell,
        // sell, replace with a buy


    }
}

impl GridStrategy {
    
    fn load_grid_params() -> HashMap<String,GridParam>{
        let mut grid_params = HashMap::new();
        grid_params.insert("BTCUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"BTCUSDT".to_string()});
        // grid_params.insert("ETHUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"ETHUSDT".to_string()});
        // grid_params.insert("BNBUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"BNBUSDT".to_string()});
        // grid_params.insert("ADAUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"ADAUSDT".to_string()});
        // grid_params.insert("DOTUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"DOTUSDT".to_string()});
        // grid_params.insert("XRPUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"XRPUSDT".to_string()});
        // grid_params.insert("DOGEUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"DOGEUSDT".to_string()});
        // grid_params.insert("UNIUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"UNIUSDT".to_string()});
        // grid_params.insert("LINKUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"LINKUSDT".to_string()});
        // grid_params.insert("LTCUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"LTCUSDT".to_string()});
        // grid_params.insert("BCHUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"BCHUSDT".to_string()});
        // grid_params.insert("SOLUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"SOLUSDT".to_string()});
        // grid_params.insert("MATICUSDT".to_string(),GridParam{amount:0.01,spread:0.01,symbol:"MATICUSDT".to_string()});
        
        return grid_params;
        
    }

    fn replace_order(&self, order: &Order) -> Trade {
        match order.side {
            Side::Buy => Trade {
                exchange: self.exchange.clone(),
                symbol: order.symbol.clone(),
                side: Side::Sell,
                amount: order.vol,
                price: order.price + self.grid_params.get(&order.symbol).unwrap().spread,
                strategy: STRATEGY_NAME.to_string(),
            },
            Side::Sell => Trade {
                exchange: self.exchange.clone(),
                symbol: order.symbol.clone(),
                side: Side::Buy,
                amount: order.vol,
                price: order.price - self.grid_params.get(&order.symbol).unwrap().spread,
                strategy: STRATEGY_NAME.to_string(),
            },
        }

    }
}
    
