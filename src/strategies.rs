use crate::strategies::grid::GridStrategy;

mod grid;

pub enum Strategies {
    Grid,
}

trait Strategy {
    fn new() -> Self;
    fn setup(&self);
    fn on_tick(&self);

}


pub fn run_stategy(strategy: Strategies) {

    let stat_to_run = match strategy {
        Strategies::Grid => {
            println!("You have selected the grid bot");
            GridStrategy::new()
            
        }
    }; 
    stat_to_run.setup();
    loop {
        todo!("Check the cmd channel");
        stat_to_run.on_tick();
    }

}