fn find() -> Option<i32>
{
//	-1
//	None
	Some(5)
}

fn main()
{
	let ret = find();

	match ret
	{
		Some(n) => println!("{}", n),
		None    => println!("fail"),
	}

}
