// assert!(), debug_assert!()
fn main()
{
	let v1 = 10;
	let v2 = 10;

	// ❶ assert : "실행 시간에" 값의 유효성 조사
	// => () 안의 표현식이 거짓일 경우 panic!() 발생
//	assert!(v1 == v2);
//	assert_eq!(v1 * 2, 20);
//	assert_ne!(v1 * 2, 20);


	// ❷ debug_assert : debug 버전에서만 동작
	//				     debug_assertions 가 정의된 경우만 동작.
	debug_assert!(v1 == v2);
	debug_assert_eq!(v1 * 2, 20);
	debug_assert_ne!(v1 * 2, 20); // panic!(), 단, 최적화 하지 않을 때만

	// => rustc 컴파일러 최적화 옵션
	// -C opt-level=0       : debug_assertions 이 정의 되어있음. 디폴트 값
	// -C opt-level=1 또는 2 : debug_assertions 이 정의 안됨.

	// ❸ 컴파일 옵션에 따른 debug_assert!() 동작 여부
	// rustc macro4.rs
	// macro4.exe 		=> debug_assert!() 동작함

	// rustc macro4.rs -C opt-level=1
	// macro4.exe 		=> debug_assert!() 동작안함

	// rustc macro4.rs -C opt-level=1 -C debug_assertions
	// macro4.exe 		=> debug_assert!() 동작함.
}