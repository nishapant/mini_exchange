// mod trade;
// mod orderbook;
mod esb;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str;
use std::time::Duration;
use std::thread;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

// https://github.com/rust-lang/rustlings/blob/master/exercises/threads/threads1.rs
struct JobStatus {
    jobs_completed: u32,
}

fn main() {

    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    let (client_sender, client_receiver) : (Sender<&str>, Receiver<&str>) = mpsc::channel();


    thread::spawn(|| handle_tcp_connection("localhost", 8881, client_receiver));

    thread::spawn(move || {
        // threads can add stuff to channel that needs to be sent across 
        let thread_client_sender = client_sender.clone();
        thread_client_sender.send("hello").unwrap();
        let msg = "loop iteration";
        thread_client_sender.send(msg).unwrap();
        let msg = "this is the third messages";
        thread_client_sender.send(msg).unwrap();
        for i in 0..5 {
            println!("in loop");
            thread::sleep(Duration::from_millis(100));
        }
        let msg = "this is the fourth";
        thread_client_sender.send(msg).unwrap();
        let mut status_shared = status_shared.lock().unwrap();
        status_shared.jobs_completed += 1;
    });


    let mut jobs_completed: u32;
    loop {
        jobs_completed = status.lock().unwrap().jobs_completed;
        if jobs_completed < 2 {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }
}

// this should be used to handle each individual connection with the matchine engine, dropcopy, tickerplant
fn handle_udp_connection(udp_host: &str, udp_port: i32) {
    // let connection_string = format!("{}:{}", udp_host, udp_port);
    // let socket = UdpSocket::bind(connection_string).expect("couldn't bind to address");

    
}

// this should be used to handle client connections
fn handle_tcp_connection(tcp_host: &str, tcp_port: i32, msg_channel: Receiver<&str>) {

    let connection_string = format!("{}:{}", tcp_host, tcp_port);
    match TcpStream::connect(connection_string) {
        Ok(mut stream) => {
            // the connection was successful
            println!("successfully connected to {}:{}", tcp_host, tcp_port);
            loop {
                let d = Duration::from_millis(10);
                // check channel to see if theres something that needs to be sent to client
                let new_msg = msg_channel.recv_timeout(d);
                if  new_msg.is_ok() {
                    // send the message back to the client
                    stream.write(new_msg.ok().unwrap().as_bytes()).unwrap();
                    println!("sent message: {}", new_msg.ok().unwrap());
                }
    
                let mut data = [0 as u8; 256]; // using 256 byte buffer
                match stream.read(&mut data) {
                    Ok(_) => {
                        println!("recevied data: {}", str::from_utf8(&data).unwrap());
                    },
                    Err(e) => {
                        // error 
                    }
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
