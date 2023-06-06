fn foo() -> i32
{
	return 10;
}

fn main()
{
	let n1 = 3;
	let n2 = (n1 + 2) * foo();

	foo();

	let n3 = {3};	
	let n4 = {3;};  

	println!("{:?}", n3); // 3
	println!("{:?}", n4); // Void ()
}