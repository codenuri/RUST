fn main()
{
	let mut i = 0;

	// ➊ 기본 모양
	while i < 10
	{
		println!("{}", i);
		i += 1; // ++i
	}

	// ➋ break, continue
	i = 0;
	while i < 10
	{
		i += 1;
		if i == 10 	  { break;}
		if i % 2 == 0 { continue;}
		println!("{}", i);
	}
}