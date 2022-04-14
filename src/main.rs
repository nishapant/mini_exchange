// mod trade;
// mod orderbook;
mod esb;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::str;
use std::time::Duration;
use std::thread;
use serde::{Serialize, Deserialize};


fn main() {
    match TcpStream::connect("localhost:8881") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8881");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");

    thread::spawn(|| {

        // everything in here runs
        // in its own separate thread
        println!("hello");
        for i in 0..5 {

            println!("Loop 2 iteration: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::sleep(Duration::from_millis(3000));
    println!("main thread waited")
}


// this should be used to handle each individual connection with the matchine engine, dropcopy, tickerplant, and gateway
fn handle_connection(tcp_host: &str, tcp_port: i32) {

    let connection_string = format!("{}:{}", tcp_host, tcp_port);
    match TcpStream::connect(connection_string) {
        Ok(mut stream) => {
            // the connection was successful
            println!("Successfully connected to {}:{}", tcp_host, tcp_port);

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },


        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderType  {
    Market
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
