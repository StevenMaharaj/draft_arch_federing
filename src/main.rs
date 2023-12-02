mod ftypes;
mod exchange1;
mod strategies;
mod flog;

use std::error::Error;

use tokio::sync::oneshot;
use exchange1::order_feed as e1;
use strategies::{run_stategy, Strategies};


#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    
    
    flog::get_logger()?;
    let strategy = Strategies::Grid;
    
    
    
    let (tx, rx) = oneshot::channel::<ftypes::Order>();
    
    
    
    
    tokio::spawn(async move {
        
        run_stategy(strategy);
        let order_feed = e1::OrderFeed::new(tx);
    });

    Ok(())


}
