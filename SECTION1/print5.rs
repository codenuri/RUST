fn main()
{
	// ❶ string literal 만 출력 가능. 문자열 변수를 직접 출력할수는 없음
	let s = "hello";

	println!("hello");	// hello
	println!(s);		// error
	println!("{}", s);	// ok
	
	// ❷ print vs eprint
	print!("hello");  	// stdout	
	println!("hello"); 

	eprint!("hello");  	// stderr
	eprintln!("hello"); 
}