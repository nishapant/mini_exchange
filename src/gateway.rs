use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::net::UdpSocket;
use bincode;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::env;
use crate::trade::{Trade, TradeType, OrderType};
use crate::trade::OrderType::{Limit, Market};
use crate::trade::TradeType::{Buy, Sell};

// https://github.com/rust-lang/rustlings/blob/master/exercises/threads/threads1.rs
struct JobStatus {
    jobs_completed: u32,
}

pub fn start_gatway() {
    let args: Vec<String> = env::args().collect();
    let mut ip_addrs = HashMap::new();

    ip_addrs.insert(1, "192.168.50.106:8082");
    ip_addrs.insert(2, "192.168.50.107:8083");
    ip_addrs.insert(3, "192.168.50.108:8084");

    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    let (trader1_sender, trader1_receiver) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let (trader2_sender, trader2_receiver) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let (trader3_sender, trader3_receiver) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let (udp_sender, udp_receiver) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();


    thread::spawn(move || handle_tcp_connection(ip_addrs.get(&{1}).unwrap(), trader1_receiver, udp_sender));


    // mimic the OME sending messages to the trader
    thread::spawn(move || {
        // threads can add stuff to channel that needs to be sent across 
        let thread_client_sender = trader1_sender.clone();
        let trade = create_trade();
        let encoded: Vec<u8> = bincode::serialize(&trade).unwrap();
        thread_client_sender.send(encoded);
        // thread_client_sender.send("hello".to_string()).unwrap();
        // thread::sleep(Duration::from_millis(200));

        // let msg = "loop iteration";
        // thread_client_sender.send(msg.to_string()).unwrap();
        // thread::sleep(Duration::from_millis(200));

        // let msg = "this is the third messages";
        // thread_client_sender.send(msg.to_string()).unwrap();
        // for i in 0..5 {
        //     println!("in loop");
        //     thread::sleep(Duration::from_millis(100));
        // }
        // let msg = "this is the fourth";
        // thread_client_sender.send(msg.to_string()).unwrap();
        let mut status_shared = status_shared.lock().unwrap();
        status_shared.jobs_completed += 1;
    });

    // thread::spawn(|| handle_udp_connection("localhost", 8080, matching_engine_receiver));

    let mut jobs_completed: u32;
    loop {
        jobs_completed = status.lock().unwrap().jobs_completed;
        if jobs_completed < 3 {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }
}

// this should be used to handle each individual connection with the matchine engine, dropcopy, tickerplant
// one to one connection
fn handle_udp_connection(udp_host: &str, udp_port: i32, msg_channel: Receiver<Vec<u8>>) {   
    let remote_addr = format!("{}:{}", udp_host, udp_port);
    println!("trying to connect");
    let local_addr = format!("192.168.50.105:{}", udp_port);

    let socket = UdpSocket::bind(local_addr.parse::<SocketAddr>().unwrap()).unwrap();
    match socket.connect(remote_addr) {
        Ok(_) => {
            println!("successfully connected");
            let d = Duration::from_millis(10);
            socket.set_read_timeout(Some(d));
            socket.set_write_timeout(Some(d));

            loop {
                let d = Duration::from_millis(10);
                // check channel to see if theres something that needs to be sent to client
                let new_msg = msg_channel.recv_timeout(d);
                if  new_msg.is_ok() {
                    // send message in stream to connection
                    // shouldn't this represent sending a message over a connection 
                    let msg_to_send = new_msg.as_ref().ok().unwrap();
                    let decoded: Trade = bincode::deserialize(&msg_to_send).unwrap();
                    socket.send(msg_to_send);
                    println!("sent message: {:?}", decoded);
                }

                let mut data = [0 as u8; 512]; // using 512 byte buffer
                match socket.recv_from(&mut data) {
                    Ok( (mut number_of_bytes, mut src_addr) ) => {
                        let filled_buf = &mut data[..number_of_bytes];
                        // check trader id -> based on trader id, put on the correct channel for the trader
                        // println!("recevied data: {}", filled_buf);
                    },
                    Err(e) => {
                        // println!("failed to send message: {}", e);
                    }
                }
            
            }
        },
        Err(e) => {
            println!("failed to connect: {}", e);
        }
    }
}

// this should be used to handle client connections
fn handle_tcp_connection(tcp_ip_addr: &str, msg_channel_receiver: Receiver<Vec<u8>>, udp_sender: Sender<Vec<u8>>) {
    match TcpStream::connect(tcp_ip_addr) {
        Ok(mut stream) => {
            println!("what's up");
            // the connection was successful
            stream.set_nonblocking(true).expect("set to non-blocking");
            println!("successfully connected to {}", tcp_ip_addr);
            loop {
                let d = Duration::from_millis(10);
                // check channel to see if theres something that needs to be sent to client
                let new_msg = msg_channel_receiver.recv_timeout(d);
                if  new_msg.is_ok() {
                    // send message in stream to connection 
                    let msg_to_send = new_msg.as_ref().ok().unwrap();
                    let decoded: Trade = bincode::deserialize(&msg_to_send).unwrap();
                    stream.write(msg_to_send).unwrap();
                    println!("sent message: {:?}", decoded);
                }
                
                let mut data = [0 as u8; 512]; // using 512 byte buffer



                // read message in connection stream
                match stream.read(&mut data) {
                    Ok(_) => {
                        // add to udp channel 
                        // TODO: deserialize and validate struct
                        if str::from_utf8(&data).unwrap().eq("") {
                            let mut data_to_send: Vec<u8> = data.to_vec();
                            udp_sender.send(data_to_send).unwrap();
                            println!("recevied data: {}", str::from_utf8(&data).unwrap());
                        }
                        
                        // unserialize the message 
                    },
                    Err(e) => {
                        // error 
                        // println!("there was an error reading");
                        // continue;
                    }
                }
            }
        },

        Err(e) => {
            println!("failed to connect: {}", e);
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn create_trade() -> Trade {
    let mut new_trade : Trade = Trade {
        trader_id: 0, //the gateway should set this based on the trader ip address
        // needs to know own ID
        order_id: 0, // always hardcode to 0
        stock_id: 0, // hardcode to 0, not checked
        trade_type: Buy, // second argument
        order_type: Limit, // 3rd argument
        unit_price: 3, // 4th argument
        qty: 1, // 5th arg
        partial_fill: true, // always will partial fill in OME
        expiration_date : 0 // unused as well, just set to 0
    };

    return new_trade;
}

fn is_valid_order (trade_: trade::Trade) -> bool {
    // make sure the order type is correctly set
    if trade_.trade_type!=trade::TradeType::Buy && trade_.trade_type!=trade::TradeType::Sell {
        return false;
    }

    // make sure OrderType is valid
    if trade_.order_type!=trade::OrderType::Market && trade_.order_type!=trade::OrderType::Limit {
        return false;
    }

    // check quantity valid
    if trade_.qty<=0 {
        return false;
    }

    return true;
}

