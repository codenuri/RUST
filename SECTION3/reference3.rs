fn main()
{
	let n = 10;

	// ❶ 레퍼런스를 만드는 방법
	let r1 : &i32 = &n;
	let r2        = &n;	


	// ❷ 레퍼런스를 사용해서 대상체 접근
	let v1 : i32 = *r1; // ok
//	let v2 : i32 = r1;  // error. 

	let v3 = *r1; // v3는 i32
	let v4 = r1;  // v4는 &i32

	// ❸ println 으로 출력할때
	println!("{:p}", &n);  // n의 주소
	println!("{}",   *r1); // 10
	println!("{}",   r1);  // 10
	println!("{:p}", r1);  // n의 주소
//	println!("{:p}", *r1); // error 
	println!("{:p}", &r1); // r1자체의 주소
	println!("{:p}", &*r1);// n의 주소	
}
