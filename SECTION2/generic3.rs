
fn log<T, U>(a : T, b : U)
	where T : std::fmt::Debug, 
		  U : std::fmt::Debug,
{
	println!("{:?}, {:?}", a, b);
}

fn main()
{
	log(3, 3.4);
}