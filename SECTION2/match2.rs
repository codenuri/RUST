fn main()
{
	let v1 = 10;
	let v2 = 20;
	let t = (10, 20);

	// ➊ tuple matching
//	match (v1, v2)
	match t
	{
		(10, 10) => println!("10, 10"),
		(10, 20) => println!("10, 20"),
		_        => println!("......"),
	}

	match t
	{
		(a, 10) => println!("10, 10"),
		(a, 20) => println!("{}, 20", a),
		_       => println!("......"),
	}	

	// ➋ match guard
	let t2 = (15, 20);
	match t2 
	{
		(a, b) if a > 10 => println!("1st arm"),
		_       => println!("......"),
	}

	// ➌ @ 표기법
	let n3 = 3;
	match n3
	{
		num @ 3 => println!("{}", num),
		_       => println!("......"),
	}	


	// ➍ array match
	let arr = [1,2,3];

	match arr 
	{
		[a, b, 3] => println!("1"),
		[a, b, c] => println!("2"),
	}	
}