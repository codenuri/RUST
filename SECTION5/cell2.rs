use std::cell::Cell;

fn main()
{
	let c1 : Cell<i32> = Cell::from(10);

	c1.set(20);

	println!("{}",c1.get());
}