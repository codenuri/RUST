// preview 8. 표준 라이브러리 사용 - 다양한 함수, 타입을 "Crates" 와 "Module" 이라는 개념으로 제공 
// use std::cmp::max; 

fn main()
{
	// ❶ 표준라이브러리 사용
	let ret1 = std::cmp::max(10, 5); // std 라는 이름의 "Crate" 안에 있는
									 // cmp "Module" 안에 있는
									 // max "Function" 사용 
	// ❷ use 사용
	use std::cmp::max;   
	let ret2 = max(10, 5);

	println!("{ret1}, {ret2}"); 
	
	// ❸ 표준 라이브러리에는 함수뿐 아니라 다양한 타입도 제공
	let list1 = std::collections::LinkedList::from([1, 2, 3]);
											// ^ LinkedList 타입 안에 있는 
											//   연관 함수 (associated function) 호출
											//   (다른 언어의 static method 와 유사한 )
	use std::collections::LinkedList;
	let list2 = LinkedList::from([1, 2, 3]);
}
