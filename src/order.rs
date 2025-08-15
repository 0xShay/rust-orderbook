#[derive(Debug)]
pub enum OrderType {
	BUY,
	SELL,
}

#[derive(Debug)]
pub struct Order {
	pub order_type: OrderType,
	pub size: i32,
	pub filled: i32,
	pub price: i32,
}

impl Order {
	pub fn new(order_type: OrderType, size: i32, filled: i32, price: i32) -> Order {
		Order {
			order_type,
			size,
			filled,
			price
		}
	}
}