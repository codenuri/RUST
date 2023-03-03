fn main()
{
	let mut n = 10;

	let r1 = &mut n; // <= 대여 시작

	n = 20;
	
	*r1 = 20;		// <= 반납	
	
//	n = 20;
}