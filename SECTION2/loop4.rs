fn main()
{
	let mut i = 0;
	let mut sum = 0;

	let value = loop 
//	let value = while true
				{
					i += 1;
					sum += i;
					if i == 10
					{
						break sum;
					}
				};

	println!("{:?}", value);
}