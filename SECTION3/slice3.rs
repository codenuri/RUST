
fn main()
{
	let v = vec![0,1,2,3,4];

	let r2;
	{
		let r1 = &v[0..4];
		r2 = &r1[1..3];
	}

	println!("{:p}", v.as_ptr());
//	println!("{:p} {:?}", r1, r1);
	println!("{:p} {:?}", r2, r2);


}	
