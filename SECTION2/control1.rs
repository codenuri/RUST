fn main()
{
	let score = 75;

	let result1 = if score > 70 
				  {"pass"} else {"fail"};

	let result2 = match score 
				  {
						70 ..= 100 => "pass",
						_          => "fail",
				  };

	println!("{}, {}", result1, result2);

	if score > 75
	{
	} 	// <== ; 필요 없음	
}