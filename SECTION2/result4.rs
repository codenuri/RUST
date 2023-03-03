fn foo(x : i32) -> Result<i32, i32>
{
	if x < 0 { Err(x) }
	else { Ok(x) }
}

fn main()
{
	let ret1 = foo(30).is_ok();	
	let ret2 = foo(30).is_err();
	let ret3 = foo(30).ok();  
	let ret4 = foo(30).err(); 

	let value1 = foo(10).unwrap();
	let value2 = foo(-20).expect("panic message");

	println!("{}, {}", value1, value2);

}