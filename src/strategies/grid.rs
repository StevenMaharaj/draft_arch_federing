use super::Strategy;
use std::collections::HashMap;
use serde::Deserialize;

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
    grid_params: HashMap<String,GridParam>


}

impl Strategy for GridStrategy {
    
    fn new(exchange:String,symbols:Vec<String>) -> Self {
        
        let grid_params =  load_grid_params();
        Self {exchange,symbols,grid_params  }
        
    }


    fn on_tick(&self) {
        todo!()
    }
}

    fn load_grid_params() -> HashMap<String,GridParam>{
        todo!();
    }
    
