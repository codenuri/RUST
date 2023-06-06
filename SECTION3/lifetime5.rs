fn f1(x : &'static i32 )
{
}

fn main()
{
	let n = 10;
	static N : i32 = 10;

//	f1(&n);
	f1(&N);
}
