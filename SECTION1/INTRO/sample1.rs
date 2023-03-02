// ❶ rustc sample.rs -A unused_variables
//    rustc sample.rs -A unused_variables -A dead_code
//    rustc sample.rs -A unused_variables -A dead_code -o out.exe

// ❷ 컴파일러 옵션 보기 : rustc -h 

// ❸ "The rustc book" 참고

fn main()
{
	// v1 변수 선언
	let v1 = 10;	
}

fn foo() {}