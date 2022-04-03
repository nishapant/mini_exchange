use std::collections::{HashMap};
use crate::trade::Trade;


// add
// cancel -> order ID
pub struct OrderBook {
    book: HashMap<u32, Trade>,
    bids: Vec<Trade>, //people who are buying -> should be least to greatest
    asks: Vec<Trade>, //people who are selling -> should be greatest to least
}

impl OrderBook {
    pub fn new(input: Trade) -> Self {
        Self {
            book: HashMap::new(),
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    // What can we really change here?
    // Just the unit_price and qty?
    pub fn modify (&mut self, trade_input: Trade) -> Result<T, E> {
        //can take the modified trade and match it with the id number of the trade to modify
        let mut trade = self.book.get(&trade_input.order_id).unwrap();
        self.book.remove(&trade_input.order_id);
        self.book.insert(trade_input.order_id, trade_input);

        if trade_input.trade_type == True {

        } else {

        }

        return Ok();
    }

    fn get_bid_insert_idx(mut vec: Vec<Trade>, trade: Trade) -> usize {
        for i in 0..vec.len() - 1 {
            if vec[i].unit_price <= trade.unit_price && vec[i + 1].unit_price > trade.unit_price  {
                return i+1;
            }
        }

        return vec.len() - 1;
    }

    fn get_ask_insert_idx(mut vec: Vec<Trade>, trade: Trade) -> usize {
        for i in 0..vec.len() - 1 {
            if vec[i].unit_price >= trade.unit_price && vec[i + 1].unit_price < trade.unit_price  {
                return i+1;
            }
        }

        return 0;
    }

    fn insert_and_remove(mut vec: Vec<Trade>, input_trade: Trade, trade_type: bool) {
        // the vector to insert into, the trade id to add (same as remove)
        if trade_type {
            let mut flag = false;
            for i in 0..vec.len() - 1 {
                if vec[i].order_id = input_trade.order_id {
                    vec.remove(i);
                }
                if vec[i].unit_price > trade.unit_price && vec[i + 1].unit_price < trade.unit_price  {
                    vec.insert(i, trade.clone());
                    flag = true;
                }

            }
            if !flag {
                vec.insert(vec.len() - 1, input_trade.clone());
            }
        } else {

        }

    }

    //needs testing xd!
    pub fn add (&mut self, trade: Trade) -> Result<T, E> {
        //insert into hashmap and then add to the appropriate array
        self.book.insert(trade.order_id,trade);
        //true is bid(buyers) and false is ask(seller)
        if Trade.trade_type == true {
            let mut flag = false;
            for i in 0..self.bids.len() - 1 {
                if self.bids[i].unit_price > trade.unit_price && self.bids[i + 1].unit_price < trade.unit_price  {
                    self.bids.insert(i, trade.clone());
                    flag = true;
                }
            }
            if !flag {
                self.bids.insert(self.bids.len() - 1, trade.clone());
            }
        }

        // else {
        //     let mut flag = false;
        //     for i in 0..self.bids.len() - 1 {
        //         if self.bids[i] < trade.unit_price && self.bids[i + 1].unit_price > trade.unit_price  {
        //             self.bids.insert(i, trade.clone());
        //             flag = true;
        //         }
        //     }
        //     if !flag {
        //         self.bids.insert(self.bids.len() - 1, trade.clone());
        //     }
        // }
        return Ok();
    }

    pub fn remove (&mut self, order_id: u32) {
        // query the hashmap get the Trade info
        let mut trade = self.book.get(&order_id).unwrap();
        //remove from the bid/ask vector
        if trade.trade_type == true {
            for i in 0..self.bids.len() {
                if order_id == trade.order_id {
                    self.bids.remove(i);
                }
            }

        } else {
            for i in 0..self.asks.len() {
                if order_id == trade.order_id {
                    self.asks.remove(i);
                }
            }
        }
        //remove from the hashmap
        self.book.remove(&order_id);
    }

    //Could create and return a spread struct that contains bid and ask, not sure about the best implimentation
    pub fn top (&self) -> (u32, u32) {
        let bid = self.bids[0].unit_price.clone();
        let ask = self.asks[0].unit_price.clone();
        return (bid, ask);
    }
}

