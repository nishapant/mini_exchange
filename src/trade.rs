#[readonly::make]
#[derive(Debug, Clone)]
pub struct Trade {
    // Customer ID (a unique identifier for the market customer)
    // Stock ID (usually the CUSIP number)
    // Order type (buy/sell)
    // Order_type (market, limit, stop)
    // Unit price (i.e., target price for a limit order)
    // Number of shares (to be bought or sold)
    // Partial fill allowed (for limit orders): 1 = partial fill allowed, 0 = no partial fill
    // Expire time (time the order should be active in the market, in minutes)
    trader_id: u32,
    stock_id: u32, //we could leave this blank and assume that our exchange only trades one asset type
    #[readonly]
    pub order_id: u64, //number assigned by the gateway that is sent back to the trader and used to edit/cancel orders
    #[readonly]
    pub trade_type: bool, //buy or sell
    pub Order_type: OrderType, //What type of order (market, limit, etc)
    pub unit_price: f64, //price of share
    pub qty: u32, //number of the item they want to buy or sell 
    partial_fill: bool, //is partial fill of orders allowed or not
    expiration_date: u32, //immediate fill, end_of_day, 90 day? unsure what common types there are
}

#[derive(Debug, Clone)]
pub enum OrderType  {
    Market
}