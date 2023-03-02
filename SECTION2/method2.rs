fn main()
{
	let n1 : i32 = -10;
	
	// n1의 절대값을 출력 하고 싶다.
	println!("{}", n1.abs() );


	// 24와 18의 최대 공약수를 찾고 싶다.
	println!("{}", 24_u32.rem_euclid(18) );
}