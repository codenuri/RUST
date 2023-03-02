fn size_of<T>()
{
	println!("{}", std::mem::size_of::<T>());	
}

fn main()
{
//	size_of();
//	size_of<i32>(); // C#, C++
	size_of::<i32>();
}
