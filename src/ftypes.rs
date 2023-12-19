

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone,)]
pub struct Order{
    pub price: f32,
    pub vol:f32,
    pub side: Side,
    pub symbol: String,
}


#[derive(Clone,)]
pub struct Trade {
    pub exchange: String,
    pub symbol: String,
    pub side: Side,
    pub amount: f32,
    pub price: f32,
    pub strategy: String,

}



impl Order {
    pub fn new(price:f32,vol:f32,side:Side,symbol: String) -> Self {
        Self { price, vol, side, symbol}
    }

    pub fn as_log(&self) -> String {
        format!("Order {:?} for {} @ {} of {}",
        self.side,
        self.vol,
        self.price,
        self.symbol)
    }
}

impl Trade {

    pub fn new(exchange:String,symbol:String,side:Side,amount:f32,price:f32,strategy:String) -> Self {
        Self { exchange, symbol, side, amount, price, strategy}
    }

    pub fn as_log(&self) -> String {
        format!("Trade {:?} for {} @ {} of {}",
        self.side,
        self.amount,
        self.price,
        self.symbol)
    }
}