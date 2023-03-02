// preview 5. 사용자 정의 타입 만들기 ( 객체지향 프로그래밍 특징 )

// ❶ struct 키워드 사용. class 키워드가 없고, 상속 문법을 지원 하지 않음.
struct Point 
{
	x : i32, // Filed	
	y : i32, // (주의) ; 이 아니고 , 	
}

// ❷ method 추가
impl Point 
{
	fn get_x(&self) -> i32 
	{
		return self.x;
	}
}

fn main() 
{	
	// ❸ 객체 생성
	let pt = Point{x:10, y:20};

	// ❹ method 호출
	let x = pt.get_x();

	println!("{x}");
}
