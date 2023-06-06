fn add(x:i32, y:i32) -> i32 { return x + y;}
fn sub(x:i32, y:i32) -> i32 { return x - y;}

fn main()
{	
	let mut f1 : fn(i32, i32)->i32 = add;
	let mut f2 = add;
	let mut f3 = add as fn(i32, i32)->i32;

	// 아래 코드에서 에러를 찾아 보세요
	f1 = sub;
	f2 = sub; // error
	f3 = sub;
}