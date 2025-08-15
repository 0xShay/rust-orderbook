use std::io;
use std::collections::BTreeMap;
use std::str::SplitWhitespace;
use std::time::SystemTime;

pub mod orderbook;

use crate::orderbook::*;

fn read_in_quantity_and_price(split: &mut SplitWhitespace<'_>, quantity: &mut i32, price: &mut Option<i32>) {
    let quantity_opt_str: Option<&str> = split.next();
    let price_opt_str: Option<&str> = split.next();
    
    // ensure quantity_str is a valid integer
    *quantity = quantity_opt_str.expect("No quantity provided.").trim().parse::<i32>().expect("Invalid quantity provided.");

    // ensure quantity is positive
    assert!(*quantity > 0, "Quantity must be positive.");

    // see if a price has been supplied
    match price_opt_str {
        None => {},
        Some(price_str) => {
            // ensure price_str is a valid number
            let price_num = price_str.trim().parse::<i32>().expect("Invalid price provided.");

            // ensure price_num is positive
            assert!(price_num > 0, "Price must be positive.");

            *price = Some(price_num);
        }
    };

    ()
}

fn main() {
    let mut input_string = String::new();

    let mut ob = Orderbook {
        bids: BTreeMap::new(),
        asks: BTreeMap::new(),
    };

    populate_orderbook(&mut ob);
    println!();

    list_orders(&ob);

    while input_string.trim() != "EXIT" {
        input_string.clear();

        println!("Enter a command:");
        io::stdin().read_line(&mut input_string).unwrap();
        println!();

        println!("===============================");

        println!();

        let start_time = SystemTime::now();

        let mut split = input_string.split_whitespace();
        
        let command: Option<&str> = split.next();

        match command.expect("No command specified.") {
            "EXIT" => {},
            "BUY" => {
                let mut quantity: i32 = 0;
                let mut price_opt: Option<i32> = None;
                read_in_quantity_and_price(&mut split.clone(), &mut quantity, &mut price_opt);

                match price_opt {
                    None => {
                        market_buy(&mut ob, quantity);
                    },
                    Some(price) => {
                        limit_buy(&mut ob, quantity, price);
                    }
                };
            },
            "SELL" => {
                let mut quantity: i32 = 0;
                let mut price_opt: Option<i32> = None;
                read_in_quantity_and_price(&mut split.clone(), &mut quantity, &mut price_opt);

                match price_opt {
                    None => {
                        market_sell(&mut ob, quantity);
                    },
                    Some(price) => {
                        limit_sell(&mut ob, quantity, price);
                    }
                };
            },
            _ => {
                println!("Unknown command.");
            }
        };

        let end_time = SystemTime::now();
        let difference = end_time.duration_since(start_time).unwrap();
        println!();
        println!("Executed in {}μs", difference.as_micros());

        println!();
        list_orders(&ob);
    }

    println!("Program terminating.");
}
