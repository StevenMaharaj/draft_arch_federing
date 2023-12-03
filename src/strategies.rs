use crate::strategies::grid::GridStrategy;

mod grid;

pub enum Strategies {
    Grid,
}

trait Strategy {
    fn new(exchange:String,symbols:Vec<String>) -> Self;
    fn on_tick(&self);

}


pub fn run_stategy(strategy: Strategies) {

    let stat_to_run = match strategy {
        Strategies::Grid => {
            println!("You have selected the grid bot");
            GridStrategy::new("EX1",
        vec!["BTC-USDT".to_string()])
            
        }
    }; 
    loop {
        todo!("Check the cmd channel");
        stat_to_run.on_tick();
    }

}