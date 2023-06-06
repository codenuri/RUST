trait Draw   { fn draw(&self); }

struct Rect { x: i32}
struct Circle{ x: i32}

impl Draw for Rect   { fn draw(&self) { println!("draw Rect");} }
impl Draw for Circle { fn draw(&self) { println!("draw Circle");} }

fn main()
{
	let n1 = 10;
	let r1 : &i32     = &n1;
	let b1 : Box<i32> = Box::new(10);

	let rect = Rect{x:1};

	let r2 : &dyn Draw     = &rect;
//	let b2 : Box<dyn Draw> = Box::new(Rect{x:1});
	let b2 : Box<dyn Draw> = Box::new(Circle{x:1});

	println!("{}", std::mem::size_of_val(&b1));
	println!("{}", std::mem::size_of_val(&b2));
}
