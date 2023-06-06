fn add(x : i32, y : i32) -> i32 { x + y }
fn sub(x : i32, y : i32) -> i32 { x - y }

fn calc(x:i32, y:i32, pf : fn(i32, i32)->i32 ) -> i32
{
	pf(x, y)
}

fn main()
{
	println!("{}", calc(5, 3, add)); // 8
	println!("{}", calc(5, 3, sub)); // 2

	println!("{}", get_op(1)(5,3)); // 8 
	println!("{}", get_op(2)(5,3)); // 2
}

fn get_op(id : i32) -> fn(i32, i32)->i32
{
	match id
	{
		1 => add,
		2 => sub,
		_ => todo!()
	}
}
