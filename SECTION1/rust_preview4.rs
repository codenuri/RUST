// preview 4. 반복문 - while, loop, for
fn main()
{	
	// ❶ imutable vs mutable 
	//  let i = 0;	// i 값을 변경할 수 없다. immutable
	let mut i = 0;	// i 값을 변경할 수 있다. mutable

	// ❷ while 문 - 조건에 따른 반복
	while i < 10
	{
		println!("{i}");
		i += 1;		// 	"++i" 는 안됨 - ++ 연산자 제공 안함
	}	

	// ❸ loop 문 - 무한 루프 ( while true 과 동일 )
	loop
	{
		// break    : 반복문 탈출
		// continue : 반복문의 처음으로 이동
		break;
	}

	// ❹ for-in : iterator 를 가진 타입(대부분 collection)의 모든 요소를 차례대로 열거
	let arr = [1,2,3,4,5];
	
	for e in arr
	{
		println!("{e}");
	}

	// 0~9 까지 반복
	for e in 0..10
	{
		println!("{e}");
	}
}

