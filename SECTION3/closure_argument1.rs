fn foo(f : fn(i32, i32)->i32 )
{
	println!("{}", f(1, 2));
}

fn main()
{
	let v = 10;

	foo( |x:i32, y:i32| x + y );
	foo( |x:i32, y:i32| x + y + v );
}
