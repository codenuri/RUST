// preview 1. variable, array, tuple, reference
fn main() 
{
	// ❶ 변수 선언
	let v1 : i32 = 10;		// 타입(i32, 32bit signed integer, 다른 언어의 int )
	let v2       = 10;		// 타입 생략 가능. 초기값 10으로 타입 추론

	println!("{v1}, {v2}");	// 변수값 출력

	// ❷ array : "같은 타입" 의 값을 여러개 보관
	let arr = [1,2,3,4,5];	
	let v3  = arr[0];	

	// ❸ tuple : "다른 타입" 의 값을 여러개 보관
	let tp = (1, 3.4, 'A');
	let v4 = tp.0;	
	let v5 = tp.1;

	// ❹ reference 변수.	
	let r1 : &i32 = &v1; // v1 를 가리키는 reference 변수 r1 선언
	let r2 = &v2;	 
}

