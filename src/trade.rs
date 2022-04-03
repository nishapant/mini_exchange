use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Trade {
    pub trader_id: u32,
    pub stock_id: u32, //we could leave this blank and assume that our exchange only trades one asset type
    pub order_id: u64, //number assigned by the gateway that is sent back to the trader and used to edit/cancel orders
    pub trade_type: bool, //buy or sell
    pub order_type: OrderType, //What type of order (market, limit, etc)
    pub unit_price: f64, //price of share
    pub qty: u32, //number of the item they want to buy or sell
    pub partial_fill: bool, //is partial fill of orders allowed or not
    pub expiration_date: u32, //immediate fill, end_of_day, 90 day? unsure what common types there are
}

#[derive(Debug, Clone)]
pub enum OrderType  {
    Market
}

// #[derive(Debug, Clone)]
// pub enum Outcome {
//     Filled {
//         order_id: u64,
//         order_type: OrderType,
//         unit_price: f64,
//         qty: u32,
//         time_stamp: SystemTime,
//     },

//     PartiallyFilled {
//         order_id: u64,
//         order_type: OrderType,
//         unit_price: f64,
//         qty: u32,
//         time_stamp: SystemTime,
//     },

//     Failed {
//         //TODO
//         //No match, duplicate, etc...
//     }, 

//     Cancelled { 
//          order_id: u64,
//          ts: SystemTime,
//     },
// }

#[derive(Debug, Clone)]
pub struct OrderUpdate {
    order_id: u64,
    order_type: OrderType,
    unit_price: f64,
    qty: u32,
    time_stamp: SystemTime,
    extradata: u32,
    status: Statuses
}


#[derive(Debug, Clone)]
pub enum Statuses {
    Filled, PartiallyFilled, Failed, Cancelled,
}