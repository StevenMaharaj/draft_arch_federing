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
    grid_params: HashMap<String,GridParam>,
    orders: HashMap<String, i32>


}

impl Strategy for GridStrategy {
    
    fn new(exchange:String,symbols:Vec<String>) -> Self {
        
        let grid_params =  load_grid_params();
        let mut orders = HashMap::new();
        symbols.iter().for_each(|s| {
            orders.insert(s.to_string(),0);
        });
        Self {exchange,symbols,grid_params,orders  }
        
    }


    fn on_tick(&self) {
        todo!()
    }
}

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
    
