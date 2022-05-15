use text_io::read;
use crate::trade::{Trade, TradeType, OrderType};
use crate::trade::OrderType::{Limit, Market};
use crate::trade::TradeType::{Buy, Sell};
use std::sync::mpsc::{Receiver, Sender};
use std::net::{TcpListener};
use std::time::Duration;
use std::io::Write;
use std::io::Read;
use std::str;

pub fn get_trade_from_client() -> Trade {
    loop {
        println!("Enter a trade type. 1:Buy 2:Sell");
        let mut temp: u64 = read!("{}\n");
        let mut trade_type: TradeType;
        if temp == 1 {
            trade_type = Buy;
        } else if temp == 2 {
            trade_type = Sell;
        } else {
            //TODO might need better logic
            println!("You have entered a wrong command please retry");
            continue;
        }
    
        println!("Enter the quantity of shares you want to trade.");
        let qty: u32 = read!("{}\n");
    
        println!("Enter an order type. 1:Limit 2:Market");
        temp: u64 = read!("{}\n");
        let order_type: OrderType;
        let mut unit_price: u64 = 0;
        if temp == 1 {
            order_type = Limit;
            println!("Enter a price per share in cents.");
            unit_price = read!("{}\n");
        } else if temp == 2 {
            order_type = Market;
        } else {
            //TODO might need better logic
            println!("You have entered a wrong command please retry");
            continue;
        }
    
        let mut new_trade : Trade = Trade {
            trader_id: 0, //the gateway should set this based on the trader ip address
            // needs to know own ID
            order_id: 0, // always hardcode to 0
            stock_id: 0, // hardcode to 0, not checked
            trade_type: trade_type, // second argument
            order_type: order_type, // 3rd argument
            unit_price: unit_price, // 4th argument
            qty: qty, // 5th arg
            partial_fill: true, // always will partial fill in OME
            expiration_date : 0 // unused as well, just set to 0
        } ;
        println!("{:?}", new_trade);
    
        return new_trade;
    }
}

pub fn start_server(curr_ip_addr: &str, msg_channel_receiver: Receiver<String>, gateway_msg_channel_sender: Sender<String>) {
    let listener = TcpListener::bind(curr_ip_addr).unwrap();
    println!("Server listening on port 8082");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // the read method will not block
                stream.set_nonblocking(true).expect("set to non-blocking");

                loop {
                    // poll the channel here for orders that need to be sent
                    let d = Duration::from_millis(10);
                    let new_msg = msg_channel_receiver.recv_timeout(d);
                    if  new_msg.is_ok() {
                        // send message in stream to connection
                        let msg_to_send = new_msg.as_ref().ok().unwrap();
                        stream.write(msg_to_send.as_bytes()).unwrap();
                        println!("sent message: {}", new_msg.ok().unwrap());
                    }    

                    // read from the stream
                    let mut data = [0 as u8; 512]; // using 50 byte buffer
                    match stream.read(&mut data) {
                        Ok(size) => {
                            gateway_msg_channel_sender.send(str::from_utf8(&data).unwrap().to_string()).unwrap();
                            // println!("recevied data: {}", str::from_utf8(&data).unwrap());
                        },
                        Err(_) => {
                            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                        }
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
