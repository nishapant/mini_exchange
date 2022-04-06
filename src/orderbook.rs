use std::collections::{HashMap, LinkedList};
use crate::trade::Trade;


//#[derive(Debug)]
//#[derive(Copy, Clone, PartialEq, Eq)]
pub struct OrderBook {
    book: HashMap<u64, Trade>, //order_id, Trade
    //https://stackoverflow.com/questions/28656387/initialize-a-large-fixed-size-array-with-non-copy-types
    prices: [Option<LinkedList<Trade>>; 18446744073709551615], //index is price, LL is trades at that price
    best_bid: (u64, u32), //first number is the price, second is the index in the linked list
    best_ask: (u64, u32),
    //bids- people who are buying -> should be least to greatest
    //asks- people who are selling -> should be greatest to least
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Filled,
    PartiallyFilled,
    Failed,
    Cancelled,
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
            best_bid: (u64::MAX, None.into()),
            best_ask: (u64::MAX, None.into()),
        }
    }


   pub fn remove(&mut self, order_id: u64) {
        //use order_id and get price from book
        let price : usize = self.book[&order_id.clone()].unit_price as usize;
        //remove from price linked list
        let mut list= self.prices[price.clone()].as_mut().unwrap();
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
    pub fn add(&mut self, trade: Trade) -> Result<T, E> {
        //insert into hashmap and then add to the appropriate arrays linked list
        self.book.insert(trade.order_id, trade);
        //true is bid(buyers) and false is ask(seller)
        let mut linked_list: LinkedList<Trade> = self.prices[trade.unit_price];
        linked_list.push_back(trade);

        return Ok(());
    }



    //TODO are we supposed we make qty & unit price modifiable?? or just price?
    pub fn modify(&mut self, order_id: u64, trade_input: Trade) -> Result<T, E> {
        //need to remove and add or just change and find new index
        self.remove(order_id);
        self.add(trade_input);
        return Ok(());
    }

    // //Could create and return a spread struct that contains bid and ask, not sure about the best implimentation
    pub fn top(&self) -> (u64, u64) {
        return (self.best_bid.0, self.best_ask.0);
    }

    //TODO match function -> should call add and remove appropriatly

}


