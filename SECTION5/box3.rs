trait Draw   { fn draw(&self); }

struct Rect { x: i32}
struct Circle{ x: i32}

impl Draw for Rect   { fn draw(&self) { println!("draw Rect");} }
impl Draw for Circle { fn draw(&self) { println!("draw Circle");} }

fn main()
{

	let mut v : Vec<Box<dyn Draw>> = Vec::new();
	
	v.push( Box::new(Rect{x:1}));
	v.push( Box::new(Circle{x:1}));
	
	v[0].draw(); // 
	v[1].draw();
}
