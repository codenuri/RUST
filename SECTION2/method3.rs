fn main()
{
	let n1 = 10;

	// ❶ 암시적 변환 허용 안됨
//	let f1 : f64 = n1;


	// ❷ 명시적 변환
	let f2 = n1 as f64;
	let f3 = f64::from(n1);
	
	let n3 = 3.4 as i32;
	
	// ❸ 문자열 => 정수 또는 실수
	let a = "10".parse::<i32>().unwrap();
	let b = "3.4".parse::<f64>().unwrap();

	
	println!("{:?}", a);
	println!("{:?}", b);	
}