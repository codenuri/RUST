struct Point
{
	x : i32,
	y : i32,	
}

fn main()
{
	let pt1 = Point{x:10, y:20};
	let pt2 = Box::new( Point{x:10, y:20} );
	
	print_type_of(&pt1); // Point
	print_type_of(&pt2); // Box<Point>

	println!("{}", std::mem::size_of_val(&pt1));
	println!("{}", std::mem::size_of_val(&pt2));
}












fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}