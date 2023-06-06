fn main()
{
	let v = vec![0,1,2,3,4];

	let r1 : &Vec<i32> = &v;
	let r2 : &[i32] = &v[1..4];
//	let r3 : &[i32] = &v[0..v.len()];
	let r3 : &[i32] = &v[..];


	println!("{:?}", v);  // [0,1,2,3,4]
	println!("{:?}", r1); // [0,1,2,3,4]
	println!("{:?}", r2); // [1,2,3]
	println!("{:?}", r3); // [0,1,2,3,4]

	println!("{:p}, {:p}", &v, v.as_ptr());
	println!("{:p}", &v[0]); // 버퍼 주소
	println!("{:p}, {:p}", &r1, r1); // v주소
	println!("{:p}, {:p}", &r2, r2); // 
	println!("{:p}, {:p}", &r3, r3); // 버퍼주소

	let r4 = &v[1..3];

	print_type_of(&r4);
	
}

fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}

