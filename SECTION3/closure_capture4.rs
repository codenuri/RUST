fn main()
{
	let mut n = 10;

	let mut f = move || n = 20;

	f();

	println!("{}", n);
}
