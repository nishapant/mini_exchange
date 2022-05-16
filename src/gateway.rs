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

// https://github.com/rust-lang/rustlings/blob/master/exercises/threads/threads1.rs
struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // serialization
    let target: Option<String>  = Some("hello world".to_string());

    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let decoded: Option<String> = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(target, decoded);

    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    let (trader1_sender, trader1_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let (trader2_sender, trader2_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let (trader3_sender, trader3_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let (udp_sender, udp_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();


    thread::spawn(|| handle_tcp_connection("192.168.50.106", 8082, trader1_receiver, udp_sender));

    thread::spawn(move || {
        // threads can add stuff to channel that needs to be sent across 
        let thread_client_sender = trader1_sender.clone();
        thread_client_sender.send("hello".to_string()).unwrap();
        thread::sleep(Duration::from_millis(200));

        let msg = "loop iteration";
        thread_client_sender.send(msg.to_string()).unwrap();
        thread::sleep(Duration::from_millis(200));

        let msg = "this is the third messages";
        thread_client_sender.send(msg.to_string()).unwrap();
        for i in 0..5 {
            println!("in loop");
            thread::sleep(Duration::from_millis(100));
        }
        let msg = "this is the fourth";
        thread_client_sender.send(msg.to_string()).unwrap();
        let mut status_shared = status_shared.lock().unwrap();
        status_shared.jobs_completed += 1;
    });

    // thread::spawn(|| handle_udp_connection("localhost", 8080, matching_engine_receiver));

    // thread::spawn(move || {
    //     // threads can add stuff to channel that needs to be sent across 
    //     let thread_matching_engine_sender = matching_engine_sender.clone();
        
    //     thread_matching_engine_sender.send("udp message").unwrap();
    //     thread::sleep(Duration::from_millis(200));

    //     let mut status_shared = status_shared.lock().unwrap();
    //     status_shared.jobs_completed += 1;
    // });


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
fn handle_udp_connection(udp_host: &str, udp_port: i32, msg_channel: Receiver<String>) {   
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
                    socket.send(msg_to_send.as_bytes());
                    println!("sent message: {}", new_msg.ok().unwrap());
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
fn handle_tcp_connection(tcp_host: &str, tcp_port: i32, msg_channel_receiver: Receiver<String>, udp_sender: Sender<String>) {
    let connection_string = format!("{}:{}", tcp_host, tcp_port);
    match TcpStream::connect(connection_string) {
        Ok(mut stream) => {
            println!("what's up");
            // the connection was successful
            stream.set_nonblocking(true).expect("set to non-blocking");
            println!("successfully connected to {}:{}", tcp_host, tcp_port);
            loop {
                let d = Duration::from_millis(10);
                // check channel to see if theres something that needs to be sent to client
                let new_msg = msg_channel_receiver.recv_timeout(d);
                if  new_msg.is_ok() {
                    // send message in stream to connection 
                    let msg_to_send = new_msg.as_ref().ok().unwrap();
                    stream.write(msg_to_send.as_bytes()).unwrap();
                    println!("sent message: {}", new_msg.ok().unwrap());
                }
                
                let mut data = [0 as u8; 512]; // using 512 byte buffer

                // read message in connection stream
                match stream.read(&mut data) {
                    Ok(_) => {
                        // add to udp channel 
                        // TODO: deserialize and validate struct
                        udp_sender.send(str::from_utf8(&data).unwrap().to_string()).unwrap();
                        println!("recevied data: {}", str::from_utf8(&data).unwrap());
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
