fn main()
{
	let score = 85;

	let total = match score
				{
					90..=100 => 'A',
					80..=89 => 'B',
					70..=79 => 'C',
					60..=69 => 'D',
					_ => 'E'
				};

	println!("{:?}", total);
}