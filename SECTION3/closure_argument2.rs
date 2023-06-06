fn foo<F>(mut f : F ) 
//	where F : Fn(i32, i32) -> i32
	where F : FnMut(i32, i32) -> i32
{
	println!("{}", f(1, 2));
}

fn main()
{
	let mut v = 10;

	foo(|x:i32, y:i32| x + y);
	foo(|x:i32, y:i32| x + y + v);

	foo(|x:i32, y:i32| {v = 20; x + y + v});

	v = 0;
}