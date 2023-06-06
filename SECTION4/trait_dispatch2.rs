trait Draw   { fn draw(&self); }

struct Rect   { x : i32}
struct Circle { y : i32}

impl Draw for Rect   { fn draw(&self) { println!("draw Rect");} }
impl Draw for Circle { fn draw(&self) { println!("draw Circle");} }

fn goo( d : &dyn  Draw ) { d.draw(); }

fn main()
{
	let r = Rect{x:10};
	let c = Circle{y:10};
	

	let mut d : &dyn Draw = &r;
	d.draw();  // "draw Rect"

	d = &c;
	d.draw();  //"draw Circle"

	println!("{}", std::mem::size_of_val(&r));
	println!("{}", std::mem::size_of_val(&c));
	println!("{}", std::mem::size_of_val(&d));	
}
