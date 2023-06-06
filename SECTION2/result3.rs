fn foo(x : i32) -> Result<i32, i32>
{
	if x < 0 { Err(x) }
	else { Ok(x) }
}

fn main()
{
	let ret = foo(30);

	match ret 
	{
		Ok(v)  => println!("Ok  : {}", v),
		Err(v) => println!("Err : {}", v),
	}

	let value = match foo(30)
	{
		Ok(v)  => v,
		Err(v) => panic!("fail"),
	};

	let value = foo(-30).unwrap();
}