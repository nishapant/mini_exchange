#[readonly::make]
#[derive(Clone)]
pub struct Trade {
    trader_id: u32,
    stock_id: u32, //we could leave this blank and assume that our exchange only trades one asset type
    #[readonly]
    pub order_id: u32, //number assigned by the gateway that is sent back to the trader and used to edit/cancel orders
    #[readonly]
    pub trade_type: bool, //buy or sell
    is_market: bool, //is it a market order or is it set at a certain price
    pub unit_price: u32, //TODO
    pub qty: u32, //number of the item they want to buy or sell //TODO
    partial_fill: bool, //is partial fill of orders allowed or not
    expiration_date: u32, //immediate fill, end_of_day, 90 day? unsure what common types there are
}