fn f1( s : String )  {} // s = "ABCD"
fn f2( s : &String ) {}
fn f3( s : &str )    {}

fn f4( s : &mut String ) {}

fn main()
{
	let s = String::from("ABCD");

//	f1(s);		// ok. 문자열 move
//	f1("ABCD"); // error

//	f2(&s);		// ok
//	f2("ABCD");	// error

	f3(&s);		// ok
	f3("ABCD");	// ok

	// let r :&str = &s;
}