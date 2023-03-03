fn main()
{
	let v1 = 10;
	let v2 = 20;

//	let r1 = &v1;
	let mut r1 = &v1;

	r1 = &v2;

	println!("{}", *r1);


	let mut n = 10;
	let mut r : &mut i32 = &mut n;	
}