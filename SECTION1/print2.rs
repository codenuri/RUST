fn main()
{	
	// ❶ positional argument vs named argument	
	println!("{}, {}", 3, 1);				// 3, 1 
	println!("{0}, {1}, {0}", "A", 10);		// A, 10, A
	println!("{name}, {age}", name="john", age=20);	// john, 20


	// ❷ 변수값 출력 ( print variable )
	let v1 = 10;
	let v2 = 20;
	println!("{}, {}", v1, v2);			// 10, 20
	println!("{0}, {1}, {0}", v1, v2);	// 10, 20, 10
	println!("{v1}, {v2}");				// 10, 20


	// ❸ 주의 사항
	let arr = [1,2,3];
//	println!("{v1+v2}");	// error
//	println!("{arr[0]}")	// error
	println!("{}, {}", arr[0], v1 + v2); // 1, 30

	// ❹ {{}} 표기법
	println!("{v1}");	// 10
	println!("{{v1}}");	// {v1}
}
