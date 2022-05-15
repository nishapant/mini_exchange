#![feature(linked_list_remove)]
#![feature(type_ascription)]

use std::env;
mod trade;
mod orderbook;
mod esb;
mod client;
use std::collections::HashMap;
use text_io::read;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::str;

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

    if args.len() < 2 {
        println!("Please enter the correct arguments");
        return
    }

    let input = &args[1];

    if input == "1" || input == "ome" {
        //run ome
        println!("hi");
    } else if input == "2" || input == "client" {
        //run client
        println!("Enter the trader id (1, 2, or 3)");
        let trader_id: u64 = read!("{}\n");
        let local_ip_addr = ip_addrs.get(&{trader_id}).unwrap();
        let (client_sender, client_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();
        thread::spawn(|| client::start_server(local_ip_addr, client_receiver));

        loop {
            let trade = client::get_trade_from_client();
            let main_client_sender = client_sender.clone();
            let encoded: Vec<u8> = bincode::serialize(&trade).unwrap();
            main_client_sender.send(str::from_utf8(&encoded).unwrap().to_string()).unwrap();
            // need to sleep so the thread doesn't combine messages
            thread::sleep(Duration::from_millis(200));
        }


    }
}
