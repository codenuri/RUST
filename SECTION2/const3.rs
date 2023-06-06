fn main()
{
	let   size : usize = 10;
	const SIZE : usize = 10;

	let arr1 : [i32;10];	// ok
//	let arr2 : [i32;size]; 	// error
	let arr3 : [i32;SIZE]; 	// ok
}