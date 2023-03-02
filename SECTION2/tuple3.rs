// ➊ 함수가 한개 이상의 값을 반환하고 싶을때
fn return_two_value() -> (i32, f64)
{
	(0, 3.4)
}
fn main()
{
	let (first, second) = return_two_value();

	println!("{}, {}", first, second);

	// ➋ unit type
	let empty = (); // unit type
}

// ➌함수가 반환값이 없을때
fn no_return_value1() 
{
}
fn no_return_value2() -> ()
{
}
fn no_return_value3() -> ()
{
	()
}