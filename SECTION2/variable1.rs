fn main()
{
	// ❶ 변수를 선언하는 2가지 방법
	let v1 : i32 = 10;	// type annotation 을 표기
	let v2       = 10;  // type annotation 을 생략

	// ❷ bool
	let b1 = true;
	let b2 = false;

	// ❸ primitive 타입 외에도 표준 라이브러리 안에 
	//   다양한 타입 제공
	let s : String   = String::new();
	let v : Vec<i32> = Vec::new();		
}