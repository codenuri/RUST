fn swap( r1 : &mut i32 , r2 : &mut i32 )
{
	let tmp = *r1;
	*r1 = *r2;
	*r2 = tmp;
}

fn main()
{
	let mut x = 10;
	let mut y = 20;

	swap(&mut x, &mut y );

	println!("{}", x); 	// 20
	println!("{}", y);	// 10

}