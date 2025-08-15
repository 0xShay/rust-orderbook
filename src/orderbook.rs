use std::collections::BTreeMap;

pub struct Orderbook {
    // both of these maps map a price to a quantity
    pub bids: BTreeMap<i32, i32>,
    pub asks: BTreeMap<i32, i32>,
}

fn gen_hashtag_loop(n: usize) -> String {
    let hashtags: String = std::iter::repeat('#').take(n).collect();
    return hashtags;
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
   
    pub fn list_orders(&self) {
        println!("===============================");
        println!();
        println!("{:<5} {:>13}", "PRICE", "QUANTITY");
    
        println!();

        if self.asks.len() < 5 {
            for _ in 0..(5 - self.asks.len()) { println!(); };
        };
    
        for ask in self.asks.iter().take(5).rev() {
            println!("${:<4.2} {:>4} {}", ask.0, ask.1, gen_hashtag_loop((*ask.1).try_into().expect("Failed to format i32 as usize.")));
        };
        println!("-------------------------------");

        println!("{:.2}bps", (100.0 * ((*self.asks.first_key_value().expect("One-sided order book.").0 as f32 / *self.bids.last_key_value().expect("One-sided order book.").0 as f32) - 1.0)));

        println!("-------------------------------");
        for bid in self.bids.iter().rev().take(5) {
            println!("${:<4.2} {:>4} {}", bid.0, bid.1, gen_hashtag_loop((*bid.1).try_into().expect("Failed to format i32 as usize.")));
        };
    
        if self.bids.len() < 5 {
            for _ in 0..(5 - self.bids.len()) { println!(); };
        };

        println!();
    }   

    fn create_buy_order(&mut self, quantity: i32, price: i32) {
        println!("Placed BUY order ({} @ ${})", quantity, price);
        match self.bids.get(&price) {
            Some(prev_quantity) => {
                // add to existing order
                self.bids.insert(price, quantity + prev_quantity);
            },
            None => {
                // create new order
                self.bids.insert(price, quantity);
            }
        };
    }

    fn create_sell_order(&mut self, quantity: i32, price: i32) {
        println!("Placed SELL order ({} @ ${})", quantity, price); 
        match self.asks.get(&price) {
            Some(prev_quantity) => {
                // add to existing order
                self.asks.insert(price, quantity + prev_quantity);
            },
            None => {
                // create new order
                self.asks.insert(price, quantity);
            }
        };
    }

    pub fn market_buy(&mut self, quantity: i32) {
        println!("Performing MARKET BUY: {} @ market price", quantity);
        println!();
        let mut left_to_buy: i32 = quantity;
        let mut total_value: i32 = 0;
        while left_to_buy > 0 {
            let (p, q) = self.asks.pop_first().expect("Insufficient sell volume.");
            if q > left_to_buy {
                // push back sell order with reduced quantity
                self.asks.insert(p, q-left_to_buy);
            
                // increase total_value
                total_value += left_to_buy * p;

                println!("Bought {} @ ${:.2}", left_to_buy, p);

                // reduce left_to_buy
                left_to_buy = 0;
            } else {
                // increase total_value
                total_value += q*p;

                // reduce left_to_buy
                left_to_buy -= q;

                println!("Bought {} @ ${:.2}", q, p);
            }
        }
        println!();
        if quantity > 0 {
            println!("Bought {} at an average price of ${:.2}", quantity, ((total_value as f32) / (quantity as f32)));
        } else {
            println!("Bought 0");
        };
    }

    pub fn market_sell(&mut self, quantity: i32) {
        println!("Performing MARKET SELL: {} @ market price", quantity);
        println!();
        let mut left_to_sell: i32 = quantity;
        let mut total_value: i32 = 0;
        while left_to_sell > 0 {
            let (p, q) = self.bids.pop_last().expect("Insufficient buy volume.");
            if q > left_to_sell {
                // push back buy order with reduced quantity
                self.bids.insert(p, q-left_to_sell);
            
                // increase total_value
                total_value += left_to_sell * p;

                println!("Sold {} @ ${:.2}", left_to_sell, p);

                // reduce left_to_sell
                left_to_sell = 0;
            } else {
                // increase total_value
                total_value += q*p;

                // reduce left_to_sell
                left_to_sell -= q;

                println!("Sold {} @ ${:.2}", q, p);
            }
        }
        println!();
        if quantity > 0 {
            println!("Sold {} at an average price of ${:.2}", quantity, ((total_value as f32) / (quantity as f32)));
        } else {
            println!("Sold 0");
        };
    }

    pub fn limit_buy(&mut self, quantity: i32, price: i32) {
        println!("Performing LIMIT BUY: {} @ ${}", quantity, price);
        println!();
        let mut left_to_buy: i32 = quantity;
        let mut total_value: i32 = 0;
        let mut total_quantity: i32 = 0;

        // check the cheapest sell order - if it doesn't exist, or it does but the price is too high, create a new buy order
        while left_to_buy > 0 {
            match self.asks.pop_first() {
                None => {
                    // there are no sell orders, so create a new buy order
                    self.create_buy_order(left_to_buy, price);
                    left_to_buy = 0;
                },
                Some((p, q)) => {
                    // check if this sell order is cheap enough
                    if p <= price {
                        // cheap enough, fill order as much as possible
                        if q > left_to_buy {
                            // push back sell order with reduced quantity
                            self.asks.insert(p, q-left_to_buy);
            
                            // increase total_value
                            total_value += left_to_buy * p;
                            total_quantity += left_to_buy;

                            println!("Bought {} @ ${:.2}", left_to_buy, p);

                            // reduce left_to_buy
                            left_to_buy = 0;
                        } else {
                            // increase total_value
                            total_value += q*p;
                            total_quantity += q;

                            // reduce left_to_buy
                            left_to_buy -= q;

                            println!("Bought {} @ ${:.2}", q, p);
                        }
                    } else {
                        self.asks.insert(p, q);
                        self.create_buy_order(left_to_buy, price);
                        left_to_buy = 0;
                    }
                }
            }
        }
        println!();
        if quantity > 0 {
            println!("Bought {} at an average price of ${:.2}", quantity, ((total_value as f32) / (total_quantity as f32)));
        } else {
            println!("Bought 0");
        };
    }

    pub fn limit_sell(&mut self, quantity: i32, price: i32) {
        println!("Performing LIMIT SELL: {} @ ${}", quantity, price);
        println!();
        let mut left_to_sell: i32 = quantity;
        let mut total_value: i32 = 0;
        let mut total_quantity: i32 = 0;

        // check the most appealing buy order - if it doesn't exist, or it does but the price is too low, create a new sell order
        while left_to_sell > 0 {
            match self.bids.pop_last() {
                None => {
                    // there are no buy orders, so create a new sell order
                    self.create_sell_order(left_to_sell, price);
                    left_to_sell = 0;
                },
                Some((p, q)) => {
                    // check if this buy order is high enough
                    if p >= price {
                        // fill order as much as possible
                        if q > left_to_sell {
                            // push back buy order with reduced quantity
                            self.bids.insert(p, q-left_to_sell);
            
                            // increase total_value
                            total_value += left_to_sell * p;
                            total_quantity += left_to_sell;

                            println!("Sold {} @ ${:.2}", left_to_sell, p);

                            // reduce left_to_sell
                            left_to_sell = 0;
                        } else {
                            // increase total_value
                            total_value += q*p;
                            total_quantity += q;

                            // reduce left_to_sell
                            left_to_sell -= q;

                            println!("Sold {} @ ${:.2}", q, p);
                        }
                    } else {
                        self.bids.insert(p, q);
                        self.create_sell_order(left_to_sell, price);
                        left_to_sell = 0;
                    }
                }
            }
        }
        println!();
        if quantity > 0 {
            println!("Sold {} at an average price of ${:.2}", quantity, ((total_value as f32) / (total_quantity as f32)));
        } else {
            println!("Sold 0");
        };
    }

    pub fn populate_orderbook(&mut self) {
        self.limit_buy(5, 8);
        self.limit_buy(4, 7);
        self.limit_buy(2, 3);
        self.limit_buy(6, 8);
        self.limit_buy(5, 15);
        self.limit_buy(10, 10);
        self.limit_buy(8, 9);
        self.limit_buy(1, 2);
        self.limit_buy(12, 14);
        self.limit_buy(7, 5);

        self.limit_sell(2, 15);
        self.limit_sell(3, 16);
        self.limit_sell(3, 17);
        self.limit_sell(4, 18);
        self.limit_sell(5, 20);
        self.limit_sell(6, 18);
        self.limit_sell(9, 21);
        self.limit_sell(15, 25);
        self.limit_sell(2, 19);
        self.limit_sell(11, 22);
    }
}