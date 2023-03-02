fn twice(n : i32) -> i32 
{
	 return n * 2;
}

fn main()
{
	// ❶ [] vs get() method
	println!("{}", x1[1]);      // 2
	println!("{:?}", x1.get(1));// Some(2) 
	println!("{:?}", x1.get(5));// None 

	// ❷ map
	let x1 = [1,2,3];
//	let x2 = x1.map(인자가 한개인 함수);
//	let x2 = x1.map(twice);
	let x2 = x1.map(|n| n * 2 );

	println!("{:?}", x2); // 2, 4, 6	

}
