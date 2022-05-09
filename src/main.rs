use std::env;

// use crate::trade::Trade;
mod trade;
use trade::Trade; 
use trade::OrderType; 
use trade::TradeType;
// use trade.rs;
// mod orderbook;
// mod client; 


/**
 * 5 Args should look like the following
 * 1. trader_id - this client's trader id, should be int
 * 2. trade_type - 1 for buy, 0 for sell
 * 3. order_type - 1 for Limit, defaults to Market
 * 4. unit_price_ - price to place order at, directly as an int
 * 5. qty_ - quantity to order, directly as int
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    let trader_id_: u8 =  args[1].parse().unwrap();
    let trade_type_: u32 = args[2].parse().unwrap();
    let order_type_: u32 = args[3].parse().unwrap();
    let unit_price_: u64 = args[4].parse().unwrap();
    let qty_: u32 = args[5].parse().unwrap();
    // let partial_fill_: u32 = args[6].parse().unwrap();
    // let expiration_date_: u32 = args[6].parse().unwrap();

    let mut new_trade : Trade = Trade {
        trader_id: trader_id_, // trader_id = args1, trader
        // needs to know own ID
        order_id: 0, // always hardcode to 0
        stock_id: 0, // hardcode to 0, not checked
        trade_type: TradeType::Sell, // second argument
        order_type: OrderType::Market, // 3rd argument
        unit_price: unit_price_, // 4th argument
        qty: qty_, // 5th arg
        partial_fill: true, // always will partial fill in OME
        expiration_date : 0 // unused as well, just set to 0
    } ;
    if trade_type_==1{
        new_trade.trade_type = TradeType::Buy;
    }
    match order_type_{
        1=>new_trade.order_type = OrderType::Limit,
        _=>new_trade.order_type = OrderType::Market,
    }
    // new_trade.stock_id = args[2];
    // new_trade.order_id = args[3];
    // new_trade.trade_type = args[4];
    // new_trade.order_type = args[5];
    // new_trade.unit_price = args[6];
    // new_trade.qty = args[7];
    // if args[8]=="true"{
    //     new_trade.partial_fill = true;
    // }
    // else {
    //     new_trade.partial_fill = false;        
    // }
    // new_trade.expiration_date = args[9];

}