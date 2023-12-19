mod ftypes;
mod exchange1;
mod strategies;
mod flog;

use std::error::Error;

// use tokio::sync::mpsc;
// use exchange1::order_feed as e1;
use strategies::{run_stategy, Strategies};


#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    
    
    let r = flog::get_logger();
    match r {
        Ok(_) => {}
        Err(e) => {
            println!("Error setting up logger: {}",e);
        }
    }
    let strategy = Strategies::Grid;
    
    
    
    // let (tx, rx) = mpsc::channel::<ftypes::Order>(100);
    
    
    
    
    tokio::spawn(async move {
        println!("strat Thread spawned");
        run_stategy(strategy).await;
        // let order_feed = e1::OrderFeed::new(tx);
    }).await?;

    
    

    Ok(())


}
