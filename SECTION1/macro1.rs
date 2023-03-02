// ❶ dbg!() 매크로
fn main() 
{
	let v1 = 10;

	println!("{v1}"); // 10
	dbg!(v1);

//	let a = v1 + 20;
	let a = dbg!(v1 + 20);

//	if a == 30
	if dbg!(a == 30)
	{
		println!("a is 30");
	}
}