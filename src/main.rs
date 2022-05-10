#![feature(type_ascription)]

use std::env;
mod trade;
mod client;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::net::TcpStream;
use text_io::read;
use std::io::Read;

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
    let mut ip_addrs = HashMap::new();

    ip_addrs.insert(1, "192.168.50.106:8082");
    ip_addrs.insert(2, "192.168.50.107:8083");
    ip_addrs.insert(2, "192.168.50.108:8084");

    let input = &args[1];

    if input == "1" || input == "ome" {
        //run ome
        println!("hi");
    } else if input == "2" || input == "client" {
        //run client
        println!("Enter the trader id (1, 2, or 3)");
        let trader_id: u64 = read!("{}\n");
        let curr_ip_addr = ip_addrs.get(&{trader_id}).unwrap();
        let trade = client::get_trade_from_client();
        let listener = TcpListener::bind(curr_ip_addr).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let mut buffer = [0; 1024];

            stream.read(&mut buffer).unwrap();

            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }
    }
}
