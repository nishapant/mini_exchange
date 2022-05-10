#![feature(type_ascription)]
use std::env;
mod trade;
mod client;
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
    let input = &args[1];
    // loop {
    if input == "1" || input == "ome" {
        //run ome
        println!("hi");
    } else if input == "2" || input == "client" {
        //run client
        client::get_trade_from_client();
    }
    // }
}