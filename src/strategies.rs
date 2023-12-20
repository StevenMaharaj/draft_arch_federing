use crate::strategies::grid::GridStrategy;
use async_trait::async_trait;
mod grid;

pub enum Strategies {
    Grid,
}

#[async_trait]
trait Strategy {
    async fn new() -> Self;
    async fn on_tick(&mut self);

}


pub async fn run_stategy(strategy: Strategies) {

    // tokio::spawn(async move {
        let mut stat_to_run = match strategy {
            Strategies::Grid => {
                println!("You have selected the grid bot");
                GridStrategy::new().await
                
            }
        }; 
    // }).await;
    // sleep(Duration::from_millis(5000));
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    loop {
        // todo!("Check the cmd channel");
        // println!("On tick fn");

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        stat_to_run.on_tick().await;
    }

}