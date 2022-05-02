#![feature(linked_list_remove)]
use std::env;
mod trade;
mod orderbook;
mod esb;



fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    if input == "1" || input == "ome" {
        //run ome
        println!("hi");
    }
}
