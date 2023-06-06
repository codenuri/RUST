fn main()
{	
	// ❶ array 과 복사 가능
	let x1 = [1,2,3,4,5];
	let x2 = x1;	

	let mut x3 = [0;5];
	x3 = x2;	

	// ❷ 타입이 다른 경우는 복사(대입) 안됨
//	let mut x4 = [0, 0, 0]; 
//	x4 = x2;	// error

	f1(x1);
	f2(&x1);
}

fn f1( x : [i32;5]) {}
fn f2( x : &[i32;5]) {}


