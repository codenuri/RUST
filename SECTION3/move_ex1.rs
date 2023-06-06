fn f1(s : String) {}
fn f2(s : String) {}
fn f3(s : &String) {}

fn main()
{
	let s1 = String::from("ABCD");
	let s2 = String::from("ABCD");
	let s3 = String::from("ABCD");

	f1(s1);			
	f2(s2.clone());
	f3(&s3);

	println!("{}", s1);	// error
	println!("{}", s2);
	println!("{}", s3);
}