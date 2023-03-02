fn main()
{
	// ❶ stack vs heap
	let v1  = 10;			
	let sp1 = Box::new(10); 

	// ❷ reference
	let r1 : &i32 = &v1;
	let r2        = &v1;

	// ❸ pointer
	let p1 : *const i32 = &v1 as *const i32;
	let p2              = &v1 as *const i32;	

}
