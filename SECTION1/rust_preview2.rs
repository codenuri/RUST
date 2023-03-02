// preview 2. 함수와 클로져(Closure)

// ❶ 함수 구현. 호출 코드 보다 아래 부분에 있어도 상관없음.
fn add(a : i32, b : i32) -> i32	
{
	return a + b;
}

fn main()
{
	// ❷ 함수 호출
	let ret1 = add(10, 5); 	

	
	// ❸ 익명의 함수 "Closure", 일부 다른 언어의 "lambda"
	let sub = |a:i32, b:i32| return a - b;

	let ret2 = sub(10, 5);

	println!("{ret1}, {ret2}");
}
