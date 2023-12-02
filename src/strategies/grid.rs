use super::Strategy;
use std::collections::HashMap;

struct GridParam {

}


pub(crate) struct GridStrategy {
    exchange: String,
    symbols: Vec<String>,
    grid_params: HashMap<String,GridParam>


}

impl Strategy for GridStrategy {
    
    fn new() -> Self {
        Self {  }
        
    }

    fn setup(&self) {
        todo!()
        // Forget this function just put everything in new.
    }

    fn on_tick(&self) {
        todo!()
    }
}