fn main()
{
//	ex1();
//	ex2();
	println!("main continue");
}

// ❶ process 를 종료 하는 "함수"
fn ex1() { std::process::exit(0); }


// ❷ process 를 종료 하고 메세지와 디버깅 정보를 출력.
// => 복구 할수 없는 오류 발생시 사용
fn ex2() { panic!("복구할수 없는 에러 발생"); }


// ❸ 실행시간이 아닌 "컴파일 시간"에 에러 발생
#[cfg(not(target_os="linux"))]
compile_error!("not linux");