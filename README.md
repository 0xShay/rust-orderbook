# rust-orderbook

This repository contains the source code for a basic orderbook implementation in Rust, exposing functionality to place both BUY and SELL limit and market orders.

## Motivation
The primary motivation for writing this code was (and is) to learn the basics of Rust syntax and it's unique features, as well as expand and apply my understanding of fundamentally key trading and market dynamics.

If you spot any issues or errors within the code, or think I could've done something differently, please raise an issue or feel free to send in a PR - I'd love to learn how I could've approached things from a different perspective, and I can take these considerations into account for future projects!

## Building and running
You can run the code by cloning the repository, `cd`ing into the cloned folder, and simply running `cargo run`. This will both build and run the project in a single step.

## Usage
Once the program is running, it will populate the order book with some dummy BUY and SELL orders (`populate_orderbook`), and then print out the highest and lowest 5 BUY and SELL orders, whilst also displaying the spread between the two.

You can run one of three commands:
- `BUY [quantity] (price)` - buy `quantity` at market rate. Optional `price` parameter will place a limit BUY order.
- `SELL [quantity] (price)` - sell `quantity` at market rate. Optional `price` parameter will place a limit SELL order.
- `EXIT` -> exits the program

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.