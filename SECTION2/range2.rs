fn main()
{
	let arr = [0,1,2,3,4,5,6,7,8,9];

//	for e in arr
//	for e in &arr[2..7]
//	for e in &arr[2..]
//	for e in &arr[..7]
	for e in &arr[..]
	{
		print!("{}, ", e);
	}
}