fn main()
{
	let stdin : std::io::Stdin = std::io::stdin();	

	let mut s = String::new();

	stdin.read_line(&mut s).unwrap();

	println!("{} AA", s);
	println!("{:?}", s);

	let s2 = s.trim();

//	let n = s2.parse::<i32>().unwrap();
	let n = match s2.parse::<i32>()
	{
		Ok(n) => n,
		Err(e) => -1,
	};

	println!("{}", n);
}