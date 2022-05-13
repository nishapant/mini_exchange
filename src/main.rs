#![feature(type_ascription)]

use std::env;
mod trade;
mod client;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::net::TcpStream;
use text_io::read;
use std::io::Read;
use std::io::Write;

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

    ip_addrs.insert(1, "0.0.0.0:8082");
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
        let curr_ip_addr = ip_addrs.get(&{trader_id}).unwrap();
        // let trade = client::get_trade_from_client();
        let listener = TcpListener::bind(curr_ip_addr).unwrap();
        println!("Server listening on port 8082");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let mut data = [0 as u8; 50]; // using 50 byte buffer
                    match stream.read(&mut data) {
                        Ok(size) => {
                            // echo everything!
                            stream.write(&data[0..size]).unwrap();
                            
                        },
                        Err(_) => {
                            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
    }
}
