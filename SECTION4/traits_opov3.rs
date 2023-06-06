fn main()
{
	let s1 = "AA".to_string();
	let s2 = "BB".to_string();

//	let s3 = s1 + s2;
	let s3 = s1 + &s2;

//	println!("{}", s1);
	println!("{}", s2);
}