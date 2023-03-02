fn main()
{
	// ❶ array 타입
	let arr1 : [i32;5] = [1,2,3,4,5];
	let arr2 : [f64;3] = [1.1, 2.2, 3.3];
	let arr3 = [1,2,3,4,5];
	
	// ❷ array 크기, 메모리 크기
	let sz = 5;
//	let arr4 : [i32;sz] = [1,2,3,4,5]; // error	

	println!("{}", arr3.len());  // 5
	println!("{}", std::mem::size_of_val(&arr3));
	

	// ❸ array 를 생성하는 방법

	let good1 : [i32;5] = [1,2,3,4,5];
	let good2           = [1,2,3,4,5];
	let good3 		    = [0;5];	

//	let bad1 : [i32;5];		// 사용시 에러
//	let bad2 : [i32;5] = [1,2,3]; 		// error
//	let bad3 : [i32;5] = [1,2,3,4,5,6]; // error	
}

