use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
struct Point
{
	x:i32, y:i32
}

fn main()
{
	let rc1 = Rc::new(Point{x:1, y:2});
	let rc2 = rc1;
//	println!("{}", rc1.x); // error

	let rc1 = Rc::new(Point{x:1, y:2});
	let rc2 = rc1.clone();
	println!("{}", rc1.x);

	println!("{}", Rc::strong_count(&rc1));
	println!("{}", Rc::strong_count(&rc2));
	println!("{}", rc1.eq(&rc2));

	println!("{:p}", &*rc1);
	println!("{:p}", &*rc2);

}
