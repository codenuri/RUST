fn main()
{
	let n1 = 10;
	let n2 = 10;
	let s1 = "ABCD".to_string();
	let s2 = "ABCD".to_string();

	let f1 = 	 || println!("{}, {}", n1, s1);
	let f2 = move|| println!("{}, {}", n2, s2);

	println!("{}", std::mem::size_of_val(&f1));
	println!("{}", std::mem::size_of_val(&f2));
}
