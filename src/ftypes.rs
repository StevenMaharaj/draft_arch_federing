

pub struct Order {
    pub price: f32,
    pub vol:f32
}


impl Order {
    pub fn new(price:f32,vol:f32) -> Self {
        Self { price, vol, }
    }

    pub fn as_log(&self) -> &str {
        format!("Order {} @ {}",self.vol,self.price).as_str()
    }
}