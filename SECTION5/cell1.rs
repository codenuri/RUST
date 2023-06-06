use std::cell::Cell;
use std::borrow::Borrow;

struct Item
{
	name  : String, 
	price : u32,
	sale  : Cell<bool>
}

fn main()
{
	let pt = Point{x:1, y:2, active:Cell::from(false) };

//	pt.x = 10;

	pt.active.set(true);

	println!("{:?}", pt.active);
	println!("{:?}", pt.active.borrow());
//	println!("{}", pt.active);

}