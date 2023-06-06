fn add(x:i32, y:i32) -> i32 { return x + y;}
fn sub(x:i32, y:i32) -> i32 { return x - y;}

fn main()
{
	let mut f1 : fn(i32, i32)->i32 = add;
	let mut f2 = add;

	f1 = sub;
	f2 = sub;	
}










fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}