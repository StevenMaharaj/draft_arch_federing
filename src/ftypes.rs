

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

pub struct Order {
    pub price: f32,
    pub vol:f32,
    pub side: Side,
}


impl Order {
    pub fn new(price:f32,vol:f32,side:Side) -> Self {
        Self { price, vol, side,}
    }

    pub fn as_log(&self) -> &str {
        format!("Order {:?} for {} @ {}",self.side,self.vol,self.price).as_str()
    }
}