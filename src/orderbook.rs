
use std::collections::{HashMap, LinkedList};
use crate::trade::Trade;
use crate::trade::TradeType::Buy;


pub struct OrderBook {
    book: HashMap<u64, Trade>, //order_id, Trade

    //https://stackoverflow.com/questions/28656387/initialize-a-large-fixed-size-array-with-non-copy-types
    prices: [Option<LinkedList<Trade>>; 18446744073709551615],
    //index is price, LL is trades at that price
    bid_max: u64,
    //first number is the price, second is the index in the linked list
    ask_min: u64,
    //bids- people who are buying -> should be least to greatest
    //asks- people who are selling -> should be greatest to least
}



impl OrderBook {
    //cleanup all trades as the end of the day or need some other scheme to clean expired trades

    pub fn new(input: LinkedList<i32>) -> Self {
        const SIZE: usize = 18446744073709551615;
        const INIT: Option<LinkedList<Trade>> = Some(LinkedList::new());
        let array: [Option<LinkedList<Trade>>; SIZE] = [INIT; SIZE];
        Self {
            book: HashMap::new(),
            prices: array,
            bid_max: u64::MAX,
            ask_min: u64::MIN,
        }
    }


    pub fn remove(&mut self, order_id: u64) {
        //use order_id and get price from book
        let price: usize = self.book[&order_id.clone()].unit_price as usize;
        //remove from price linked list
        let mut list = self.prices[price.clone()].as_mut().unwrap();
        let mut i = 0;
        for element in list.iter_mut() {
            if element.order_id == order_id {
                //let mut prev = list.prev();
                list.remove(i);
                break;
            }
            i += 1;
        }
        //remove from book
        self.book.remove(&order_id);
    }


    //needs testing xd!
    //TODO how do i declare the helper methods properly so that they can remain private?
    pub fn insert(&mut self, trade: Trade) {
        //insert into hashmap and then add to the appropriate arrays linked list
        self.book.insert(trade.order_id, trade);
        //true is bid(buyers) and false is ask(seller)
        self.prices[trade.unit_price as usize].as_mut().unwrap().push_back(trade);
    }


    //TODO are we supposed we make qty & unit price modifiable?? or just price?
    pub fn modify(&mut self, order_id: u64, trade_input: Trade) {
        //need to remove and add or just change and find new index
        self.remove(order_id);
        self.insert(trade_input);
    }

    // //Could create and return a spread struct that contains bid and ask, not sure about the best implimentation
    pub fn top(&self) -> (u64, u64) {
        return (self.bid_max, self.ask_min);
    }

    //TODO match function -> should call add and remove appropriatly
    pub fn matching(&mut self, incoming_trade: Trade) {
        if incoming_trade.trade_type == Buy { // if its a buy order
            if incoming_trade.unit_price < self.ask_min {
                self.insert(incoming_trade);
            }
        } else { //if its a sell order
        }
    }
}



