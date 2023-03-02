fn f1() -> i32 
{
//	 return 10;
	 10 		// 위와 동일
//	 10;
}

fn main()
{
	let score = 30;

	let total = { let report = 30; 
				  score + report };
	
	println!("{:?}", total);

	println!("{:?}", f1() );
}


