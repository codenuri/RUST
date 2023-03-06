trait Draw   { fn draw(&self); }

struct Rect   { x : i32}
struct Circle { y : i32}

impl Draw for Rect   { fn draw(&self) { println!("draw Rect");} }
impl Draw for Circle { fn draw(&self) { println!("draw Circle");} }

fn foo( d : &impl Draw ) { d.draw(); }
fn goo( d : &dyn  Draw ) { d.draw(); }

fn main()
{
	let n = 10;
	let r = Rect{x:10};
	let c = Circle{y:10};

	foo(&r);
	foo(&c);
//	goo(&n);

	goo(&r);
	goo(&c);
}
