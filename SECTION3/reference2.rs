fn main()
{
	let v1 = vec![0,1,2,3,4];

	let r1 = &v1;
	let r2 = &v1[1..4];

	println!("{:?}", r1);
	println!("{:?}", r2);

	println!("{}", std::mem::size_of_val(&r1));//8
	println!("{}", std::mem::size_of_val(&r2));//16

}
