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

pub fn list_orders(ob: &Orderbook) {
    println!("===============================");
    println!();
    println!("{:<5} {:>13}", "PRICE", "QUANTITY");
    
    println!();

    if ob.asks.len() < 5 {
        for _ in 0..(5 - ob.asks.len()) { println!(); };
    };
    
    for ask in ob.asks.iter().take(5).rev() {
        println!("${:<4.2} {:>4} {}", ask.0, ask.1, gen_hashtag_loop((*ask.1).try_into().expect("Failed to format i32 as usize.")));
    };
    println!("-------------------------------");

    println!("{:.2}bps", (100.0 * ((*ob.asks.first_key_value().expect("One-sided order book.").0 as f32 / *ob.bids.last_key_value().expect("One-sided order book.").0 as f32) - 1.0)));

    println!("-------------------------------");
    for bid in ob.bids.iter().rev().take(5) {
        println!("${:<4.2} {:>4} {}", bid.0, bid.1, gen_hashtag_loop((*bid.1).try_into().expect("Failed to format i32 as usize.")));
    };
    
    if ob.bids.len() < 5 {
        for _ in 0..(5 - ob.bids.len()) { println!(); };
    };

    println!();
}

fn create_buy_order(ob: &mut Orderbook, quantity: i32, price: i32) {
    println!("Placed BUY order ({} @ ${})", quantity, price);
    match ob.bids.get(&price) {
        Some(prev_quantity) => {
            // add to existing order
            ob.bids.insert(price, quantity + prev_quantity);
        },
        None => {
            // create new order
            ob.bids.insert(price, quantity);
        }
    };
}

fn create_sell_order(ob: &mut Orderbook, quantity: i32, price: i32) {
    println!("Placed SELL order ({} @ ${})", quantity, price); 
    match ob.asks.get(&price) {
        Some(prev_quantity) => {
            // add to existing order
            ob.asks.insert(price, quantity + prev_quantity);
        },
        None => {
            // create new order
            ob.asks.insert(price, quantity);
        }
    };
}

pub fn market_buy(ob: &mut Orderbook, quantity: i32) {
    println!("Performing MARKET BUY: {} @ market price", quantity);
    println!();
    let mut left_to_buy: i32 = quantity;
    let mut total_value: i32 = 0;
    while left_to_buy > 0 {
        let (p, q) = ob.asks.pop_first().expect("Insufficient sell volume.");
        if q > left_to_buy {
            // push back sell order with reduced quantity
            ob.asks.insert(p, q-left_to_buy);
            
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

pub fn market_sell(ob: &mut Orderbook, quantity: i32) {
    println!("Performing MARKET SELL: {} @ market price", quantity);
    println!();
    let mut left_to_sell: i32 = quantity;
    let mut total_value: i32 = 0;
    while left_to_sell > 0 {
        let (p, q) = ob.bids.pop_last().expect("Insufficient buy volume.");
        if q > left_to_sell {
            // push back buy order with reduced quantity
            ob.bids.insert(p, q-left_to_sell);
            
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

pub fn limit_buy(ob: &mut Orderbook, quantity: i32, price: i32) {
    println!("Performing LIMIT BUY: {} @ ${}", quantity, price);
    println!();
    let mut left_to_buy: i32 = quantity;
    let mut total_value: i32 = 0;
    let mut total_quantity: i32 = 0;

    // check the cheapest sell order - if it doesn't exist, or it does but the price is too high, create a new buy order
    while left_to_buy > 0 {
        match ob.asks.pop_first() {
            None => {
                // there are no sell orders, so create a new buy order
                create_buy_order(ob, left_to_buy, price);
                left_to_buy = 0;
            },
            Some((p, q)) => {
                // check if this sell order is cheap enough
                if p <= price {
                    // cheap enough, fill order as much as possible
                    if q > left_to_buy {
                        // push back sell order with reduced quantity
                        ob.asks.insert(p, q-left_to_buy);
            
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
                    ob.asks.insert(p, q);
                    create_buy_order(ob, left_to_buy, price);
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

pub fn limit_sell(ob: &mut Orderbook, quantity: i32, price: i32) {
    println!("Performing LIMIT SELL: {} @ ${}", quantity, price);
    println!();
    let mut left_to_sell: i32 = quantity;
    let mut total_value: i32 = 0;
    let mut total_quantity: i32 = 0;

    // check the most appealing buy order - if it doesn't exist, or it does but the price is too low, create a new sell order
    while left_to_sell > 0 {
        match ob.bids.pop_last() {
            None => {
                // there are no buy orders, so create a new sell order
                create_sell_order(ob, left_to_sell, price);
                left_to_sell = 0;
            },
            Some((p, q)) => {
                // check if this buy order is high enough
                if p >= price {
                    // fill order as much as possible
                    if q > left_to_sell {
                        // push back buy order with reduced quantity
                        ob.bids.insert(p, q-left_to_sell);
            
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
                    ob.bids.insert(p, q);
                    create_sell_order(ob, left_to_sell, price);
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

pub fn populate_orderbook(ob: &mut Orderbook) {
    limit_buy(ob, 5, 8);
    limit_buy(ob, 4, 7);
    limit_buy(ob, 2, 3);
    limit_buy(ob, 6, 8);
    limit_buy(ob, 5, 15);
    limit_buy(ob, 10, 10);
    limit_buy(ob, 8, 9);
    limit_buy(ob, 1, 2);
    limit_buy(ob, 12, 14);
    limit_buy(ob, 7, 5);

    limit_sell(ob, 2, 15);
    limit_sell(ob, 3, 16);
    limit_sell(ob, 3, 17);
    limit_sell(ob, 4, 18);
    limit_sell(ob, 5, 20);
    limit_sell(ob, 6, 18);
    limit_sell(ob, 9, 21);
    limit_sell(ob, 15, 25);
    limit_sell(ob, 2, 19);
    limit_sell(ob, 11, 22);
}