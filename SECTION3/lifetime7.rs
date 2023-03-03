struct MyType<'a>
{
	data  : i32,
	refer : &'a i32
}

fn main()
{
	let n = 10;	
	let mt;
	{
	//	let n = 10;	

		mt = MyType{data:10, refer:&n};
	}
	println!("{}", mt.refer);
}