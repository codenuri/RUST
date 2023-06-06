fn main()
{
	// ❶ mutable vs immutable
	let     v1 = 10; // immutable, read 만 가능
	let mut v2 = 10; // mutable,   r/w 모두 가능


	v1 = 20; // error
	v2 = 20; // ok.


	// ❷ mut 키워드는 변수명 앞에만 표기 
	// type annotation 에는 표기 하지 않음. 
	// ( 단, reference 를 만들 때는 타입에 표기 )
	let mut v3	     = 10;
	let mut v4 : i32 = 10;	
}
