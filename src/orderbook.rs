use std::collections::{HashMap, LinkedList};
use crate::trade::{Trade, TradeType};
use crate::trade::OrderType::{Limit, Market};
use crate::trade::TradeType::{Buy, Sell};
use rand::Rng;

//TODO make book and prices only visible for tests
pub struct OrderBook {
    pub book: HashMap<u64, Trade>, //order_id, Trade

    //https://stackoverflow.com/questions/28656387/initialize-a-large-fixed-size-array-with-non-copy-types
    //index is price, LL is trades at that price
    pub prices: [Option<LinkedList<Trade>>; 256],
    //bids- people who are buying -> should be least to greatest
    bid_max: u8,
    //asks- people who are selling -> should be greatest to least
    ask_min: u8,
}



impl OrderBook {
    //cleanup all trades as the end of the day or need some other scheme to clean expired trades

    pub fn new() -> Self {
        //const SIZE: usize = 18446744073709551615; 64
        //const SIZE: usize = 4294967296; 32
        //const SIZE: usize = 65536; 16
        const SIZE: usize = 256;
        const INIT: Option<LinkedList<Trade>> = Some(LinkedList::new());
        let array: [Option<LinkedList<Trade>>; SIZE] = [INIT; SIZE];
        Self {
            book: HashMap::new(),
            prices: array,
            bid_max: u8::MAX,
            ask_min: u8::MIN,
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


    //TODO how do i declare the helper methods properly so that they can remain private?
    pub fn insert(&mut self, trade: Trade) {
        //insert into hashmap and then add to the appropriate arrays linked list
        self.book.insert(trade.order_id, trade);
        //true is bid(buyers) and false is ask(seller)
        self.prices[trade.unit_price as usize].as_mut().unwrap().push_back(trade);
    }


    pub fn modify(&mut self, order_id: u64, trade_input: Trade) {
        if trade_input.order_id != order_id {
            return; //modify fails
        }
        if trade_input.unit_price == 0  {
            self.remove(order_id);
        } else {
            self.remove(order_id);
            self.insert(trade_input);
        }
    }

    // //Could create and return a spread struct that contains bid and ask, not sure about the best implementation
    pub fn top(&self) -> (u8, u8) {
        return (self.bid_max, self.ask_min);
    }

    //TODO match function -> should call add and remove appropriately
    //am currently assuming that everything is partial fill
    // pub fn matching(&mut self, &mut incoming_trade: Trade) {
    //     if incoming_trade.trade_type == Buy { // if its a buy order
    //         let mut to_remove: Vec<Trade> = Vec::new();
    //         for linked_list in self.prices[self.ask_min].as_mut().unwrap()..self.prices[self.bid_max].as_mut().unwrap() {
    //             for current_trade in linked_list.iter_mut() {
    //                 if current_trade.trade_type == Sell {
    //                     if incoming_trade.qty > current_trade.qty {
    //                         incoming_trade.qty = incoming_trade.qty - current_trade.qty;
    //                         to_remove.push(current_trade);
    //                     } else if current_trade.qty > incoming_trade.qty {} else {
    //                         to_remove.push(current_trade);
    //                         break;
    //                     }
    //                 }
    //             }
    //
    //             if incoming_trade.qty <= 0 {
    //                 break;
    //             }
    //         }
    //         //in the case buy is greater than all of the available sells add the remaining buy to the orderbook
    //         if incoming_trade.qty > 0 {
    //             self.book.insert(incoming_trade);
    //         }
    //
    //         //remove all of the trades as needed.
    //         //return partially filled, filled, and status or current trade
    //
    //     } else { //if its a sell order
    //     }
    // }

    //TODO create another function that routes to add/modify/match based on order type
    //Enforce that route and maybe "fn top" are the only point of interaction w the order book
    //pub fn route() {
        //if the order id already exists then send it to modify?
            //if they want to cancel the order they can just set the price = 0
        //if its a market order then match regardless of price
        //else if its a limit order then try matching with given price
            //if its too large or small then don't try matching and just insert

    //}


    #[cfg(any(test, test_utilities))]
    pub fn generate_random_trade() -> Trade {
        let mut rng = rand::thread_rng();
        let num = rng.gen::<u8>();
        let mut trade_type;
        if num % 2 == 0 {
            trade_type = Buy;
        } else {
            trade_type = Sell;
        }
        Trade {
            trader_id: rng.gen::<u8>(),
            stock_id: rng.gen::<u16>(),
            order_id: rng.gen::<u64>(),
            trade_type: trade_type, //buy or sell
            order_type: Market, //What type of order (market, limit, etc)
            unit_price: rng.gen::<u8>(), //price of share in cents so we do not have to deal w super long decimals
            qty: rng.gen::<u32>(), //number of the item they want to buy or sell
            partial_fill: true, //is partial fill of orders allowed or not
            expiration_date: rng.gen::<u32>(), //immediate fill, end_of_day, 90 day? unsure what common types there are
        }
    }

    #[cfg(any(test, test_utilities))]
    pub fn generate_random_market(input_trade_type: TradeType) -> Trade {
        let mut rng = rand::thread_rng();
        let num = rng.gen::<u8>();
        Trade {
            trader_id: rng.gen::<u8>(),
            stock_id: rng.gen::<u16>(),
            order_id: rng.gen::<u64>(),
            trade_type: input_trade_type,
            order_type: Market,
            unit_price: rng.gen::<u8>(),
            qty: rng.gen::<u32>(),
            partial_fill: true,
            expiration_date: rng.gen::<u32>(),
        }
    }

    #[cfg(any(test, test_utilities))]
    pub fn generate_random_limit(input_trade_type: TradeType) -> Trade {
        let mut rng = rand::thread_rng();
        let num = rng.gen::<u8>();
        Trade {
            trader_id: rng.gen::<u8>(),
            stock_id: rng.gen::<u16>(),
            order_id: rng.gen::<u64>(),
            trade_type: input_trade_type,
            order_type: Limit,
            unit_price: rng.gen::<u8>(),
            qty: rng.gen::<u32>(),
            partial_fill: true,
            expiration_date: rng.gen::<u32>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::orderbook::OrderBook;
    use crate::trade;

    #[test]
    fn single_insert() {
        let mut book = OrderBook::new();
        let mut rand_trade = OrderBook::generate_random_trade();
        book.insert(rand_trade);
        assert_eq!(book.book.len(), 1);
        assert_eq!(book.prices[rand_trade.unit_price as usize].as_ref().unwrap().len(), 1);
    }

    #[test]
    fn many_inserts() {
        let mut book = OrderBook::new();
        for i in 1..100 {
            let mut rand_trade = OrderBook::generate_random_trade();
            book.insert(rand_trade);
            assert_eq!(book.book.len(), i);
        }
    }

    #[test]
    fn single_remove() {
        let mut book = OrderBook::new();
        let mut rand_trade = OrderBook::generate_random_trade();
        book.insert(rand_trade);
        book.remove(rand_trade.order_id);
        assert_eq!(book.book.len(), 0);
        assert_eq!(book.prices[rand_trade.unit_price as usize].as_ref().unwrap().len(), 0);
    }

    #[test]
    fn single_modify_fail() {
        let mut book = OrderBook::new();
        let mut rand_trade = OrderBook::generate_random_trade();
        let mut second_trade = OrderBook::generate_random_trade();
        book.insert(rand_trade);
        book.modify(rand_trade.order_id, second_trade);
        // the original order should remain unmodified
        assert_eq!(book.book.len(), 1);
        assert_eq!(book.prices[rand_trade.unit_price as usize].as_mut().unwrap().len(), 1);
        assert_eq!(book.prices[second_trade.unit_price as usize].as_mut().unwrap().len(), 0);
    }

    #[test]
    fn single_modify_success() {
        let mut book = OrderBook::new();
        let mut rand_trade = OrderBook::generate_random_trade();
        let mut second_trade = OrderBook::generate_random_trade();
        second_trade.order_id = rand_trade.order_id;
        book.insert(rand_trade);
        book.modify(rand_trade.order_id, second_trade);
        assert_eq!(book.book.len(), 1);
        assert_eq!(book.prices[rand_trade.unit_price as usize].as_mut().unwrap().len(), 0);
        assert_eq!(book.prices[second_trade.unit_price as usize].as_mut().unwrap().len(), 1);
    }

    #[test]
    fn single_modify_zero_success() {
        let mut book = OrderBook::new();
        let mut rand_trade = OrderBook::generate_random_trade();
        book.insert(rand_trade);
        assert_eq!(book.book.len(), 1);
        assert_eq!(book.prices[rand_trade.unit_price as usize].as_ref().unwrap().len(), 1);

        let mut second_trade = OrderBook::generate_random_trade();
        second_trade.order_id = rand_trade.order_id;
        second_trade.unit_price = 0;
        book.modify(rand_trade.order_id, second_trade);
        assert_eq!(book.book.len(), 0);
        assert_eq!(book.prices[rand_trade.unit_price as usize].as_mut().unwrap().len(), 0);
        assert_eq!(book.prices[second_trade.unit_price as usize].as_mut().unwrap().len(), 0);
    }


    //Tests for Matching engine
    //Market Buy
    //Market Sell
    //Limit Buy -> fail, success (test success w sparse and full)
    //Limit Sell -> fail, success (test success w sparse and full)
}

