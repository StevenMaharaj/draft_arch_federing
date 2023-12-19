use crate::strategies::grid::GridStrategy;
use async_trait::async_trait;
mod grid;

pub enum Strategies {
    Grid,
}

#[async_trait]
trait Strategy {
    fn new() -> Self;
    async fn on_tick(&mut self);

}


pub async fn run_stategy(strategy: Strategies) {

    let mut stat_to_run = match strategy {
        Strategies::Grid => {
            println!("You have selected the grid bot");
            GridStrategy::new()
            
        }
    }; 
    loop {
        // todo!("Check the cmd channel");
        // println!("On tick fn");
        stat_to_run.on_tick().await;
    }

}