fn main()
{
	let n1 = 10;		// copy type
	let a1 = [1,2,3];	// copy type
	let t1 = (10, 2.3);	// copy type
	let s1 = "AB".to_string();	// non-copy
	let v1 = vec![1,2,3];		// non-copy

	let n2 = n1;
	let a2 = a1;
	let t2 = t1;
//	let s2 = s1;
	let v2 = v1;

	println!("{:?}", n1);
	println!("{:?}", a1);
	println!("{:?}", t1);
	println!("{:?}", s1);
	println!("{:?}", v1);
}