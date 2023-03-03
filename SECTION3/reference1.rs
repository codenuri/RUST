//fn foo( v2 : Vec<i32> ) // v2 = v1
//fn foo( v2 : &Vec<i32> )
fn foo( v2 : &mut Vec<i32> )
{
	println!("foo : {:?}", v2);
}

fn main()
{
	let mut v1 = vec![1,2,3];

//	foo(v1);
//	foo(v1.clone());
//	foo(&v1);
	foo(&mut v1);

	println!("{:?}", v1); 
}