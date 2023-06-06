use std::rc::Rc;
use std::cell::RefCell;
#[derive(Copy, Clone)]
struct Point
{
	x:i32, y:i32
}

fn main()
{
	let mut b1 = Box::new(Point{x:1, y:2});
	b1.x = 10;

//	let mut r1 = Rc::new(Point{x:1, y:2});
//	r1.x = 10;

	let r1 = Rc::new( RefCell::new(Point{x:1, y:1}));
	let r2 = r1.clone();

	r1.borrow_mut().x = 10;
	
	println!("{}", r2.borrow_mut().x);
}
