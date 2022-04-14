// mod trade;
// mod orderbook;
mod esb;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::thread;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};


fn main() {
    // println!("Hello, world!");
    // esb_logic();

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

    let worker = ThreadWorker::new(|| {
        listening_thread()
    });
}


// this should be used to handle each individual connection with the matchine engine, dropcopy, tickerplant, and gateway
fn handle_connection(tcp_host, tcp_port) {

    connection_string = format!("{}:{}", tcp_host, tcp_port);
    match TcpStream::connect(connection_string) {
        Ok(mut stream) => {
            // the connection was successful
            println!("Successfully connected to {}", connection_string);

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
#[serde(untagged)]
pub enum OrderType  {
    Market
}

#[derive(Debug, Clone)]
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


impl Serialize for Trade {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Trade", 9)?;
        state.serialize_field("trader_id", &self.trader_id)?;
        state.serialize_field("stock_id", &self.stock_id)?;
        state.serialize_field("order_id", &self.order_id)?;
        state.serialize_field("trade_type", &self.trade_type)?;

        // order type is an enum, not sure if this needs to be changed?
        state.serialize_field("order_type", &self.order_type)?;
        state.serialize_field("unit_price", &self.unit_price)?;
        state.serialize_field("qty", &self.qty)?;
        state.serialize_field("partial_fill", &self.partial_fill)?;
        state.serialize_field("expiration_date", &self.expiration_date)?;

        state.end()
    }
}
