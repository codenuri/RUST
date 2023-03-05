fn foo(x : i32, s : String)
{
}

fn main()
{
	let n = 10;
	let mut s = "ABC".to_string();
	
	foo(n, s);
//	foo(n, s.clone());

//	s = "ABC".to_string(); 

	println!("{}", n);
	println!("{}", s);
}