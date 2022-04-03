// use std::collections::{HashMap};
// use crate::trade::Trade;


// // add
// // cancel -> order ID
// pub struct OrderBook {
//     book: HashMap<u32, Trade>,
//     bids: Vec<Trade>, //people who are buying -> should be least to greatest
//     asks: Vec<Trade>, //people who are selling -> should be greatest to least
// }

// impl OrderBook {
//     //cleanup all trades as the end of the day

//     pub fn new(input: Trade) -> Self {
//         Self {
//             book: HashMap::new(),
//             bids: Vec::new(),
//             asks: Vec::new(),
//         }
//     }


//     fn get_bid_insert_idx(mut vec: Vec<Trade>, trade: Trade) -> usize {
//         for i in 0..vec.len() - 1 {
//             if vec[i].unit_price <= trade.unit_price && vec[i + 1].unit_price > trade.unit_price {
//                 return i + 1;
//             }
//         }

//         return vec.len() - 1;
//     }

//     fn get_ask_insert_idx(mut vec: Vec<Trade>, trade: Trade) -> usize {
//         for i in 0..vec.len() - 1 {
//             if vec[i].unit_price >= trade.unit_price && vec[i + 1].unit_price < trade.unit_price {
//                 return i + 1;
//             }
//         }

//         return 0;
//     }


//     //needs testing xd!
//     //TODO how do i declare the helper methods properly so that they can remain private?
//     pub fn add(&mut self, trade: Trade) -> Result<T, E> {
//         //insert into hashmap and then add to the appropriate array
//         self.book.insert(trade.order_id, trade);
//         //true is bid(buyers) and false is ask(seller)
//         if Trade.trade_type == true {
//             self.bids.insert(self.get_bid_insert_idx(&self.bids, &trade), trade.clone());
//         } else {
//             self.asks.insert(self.get_bid_insert_idx(&self.bids, &trade), trade.clone());
//         }

//         return Ok(());
//     }

//     pub fn remove(&mut self, order_id: u32) {
//         // query the hashmap get the Trade info
//         let mut trade = self.book.get(&order_id).unwrap();
//         //remove from the bid/ask vector
//         if trade.trade_type == true {
//             for i in 0..self.bids.len() {
//                 if order_id == trade.order_id {
//                     self.bids.remove(i);
//                 }
//             }
//         } else {
//             for i in 0..self.asks.len() {
//                 if order_id == trade.order_id {
//                     self.asks.remove(i);
//                 }
//             }
//         }
//         //remove from the hashmap
//         self.book.remove(&order_id);
//     }

//     // What can we really change here?
//     // Just the unit_price and qty?
//     //TODO is this insecure because we make qty & unit price modifiable??
//     pub fn modify(&mut self, trade_input: Trade) -> Result<T, E> {
//         //need to remove and add or just change and find new index
//         self.remove(trade_input.order_id.clone());
//         self.add(trade_input);

//         return Ok(());
//     }
//     //Could create and return a spread struct that contains bid and ask, not sure about the best implimentation
//     pub fn top(&self) -> (u32, u32) {
//         let bid = self.bids[0].unit_price.clone();
//         let ask = self.asks[0].unit_price.clone();
//         return (bid, ask);
//     }
//     }


