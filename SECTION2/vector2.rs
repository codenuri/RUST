fn main()
{
	// ➊ vector 를 생성하는 방법
	let v1 : std::vec::Vec<i32> = Vec::new();
	let v2 : Vec<i32> = Vec::new();
//	let v3  		  = Vec::new(); // error
	let v4            = Vec::<i32>::new();
	let v5            = Vec::from([1,2,3]);
//	let v6 : Vec<i32> = [1,2,3].into();
	let v6 : Vec<_> = [1,2,3].into();


	// ➋ vec! 매크로 사용
	let     v7 = vec![5, 6, 7];
	let mut v8 = vec![5, 6, 7];	

}
