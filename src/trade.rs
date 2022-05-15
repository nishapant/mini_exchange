use std::time::SystemTime;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Trade {
    pub trader_id: u8,
    pub stock_id: u16, //we could leave this blank and assume that our exchange only trades one asset type
    pub order_id: u64, //number assigned by the gateway that is sent back to the trader and used to edit/cancel orders
    pub trade_type: TradeType, //buy or sell
    pub order_type: OrderType, //What type of order (market, limit, etc)
    pub unit_price: u64, //price of share in cents so we do not have to deal w super long decimals
    pub qty: u32, //number of the item they want to buy or sell
    pub partial_fill: bool, //is partial fill of orders allowed or not
    pub expiration_date: u32, //immediate fill, end_of_day, 90 day? unsure what common types there are
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderType  {
    Market,
    Limit,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TradeType  {
    Buy, 
    Sell
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub trader_id: u8,
    pub order_id: u64,
    pub order_type: OrderType,
    pub unit_price: f64,
    pub qty: u32,
    pub time_stamp: SystemTime,
    pub status: Status
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Status {
    Filled,
    PartiallyFilled,
    Failed,
    Success,
}
