fn main()
{
	let mut i = 0;
	let mut j = 0;

	'outer: while i < 10
	{
		j = 0;
		'inner: while j < 10
		{
			println!("{}, {}", i, j);
			j += 1;

			if j == 5
			{
				break 'outer;
			}
		}
		i += 1;
	}
}