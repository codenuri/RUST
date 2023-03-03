fn main()
{
	let s = String::from("ABCD");

	let ret = s.find('C');
//	let ret = s.find('X');

	println!("{:?}", ret);

	match ret
	{
		Some(n) => println!("{}", n),
		None    => println!("fail"),
	}		

}