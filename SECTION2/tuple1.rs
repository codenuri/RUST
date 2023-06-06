fn main()
{
	// ➊ array vs tuple
	let array = [1, 2, 3];
	let tuple = (3, 5u32, 3.2 );

	println!("{:p}", &array);
	println!("{:p}", &tuple);
	println!("{}", std::mem::size_of_val(&tuple));

	// ➋ tuple 타입
	let a : [i32;3] = [1, 2, 3];
	let t : (i32, u32, f64) = (3, 5u32, 3.2 );

	// ➌ tuple 의 요소 접근
	println!("{}", t.0);
	println!("{}", t.1);
	println!("{}", t.2);

	// ➍ deconstruct
	let (a, _, c) = t;

	println!("{}", a);
	println!("{}", c);	
}
