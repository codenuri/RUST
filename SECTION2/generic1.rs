
// fn is_equal<T : std::cmp::PartialEq + std::fmt::Display >(a : T, b : T)

fn is_equal<T>(a : T, b : T)
	where T : std::cmp::PartialEq + std::fmt::Display 
{
	let ret = a == b;

	println!("{} == {} is {}", a, b, ret);
}

fn main()
{
	is_equal(2, 3);
	is_equal(3, 3);
	is_equal(3.2, 3.2);

}
