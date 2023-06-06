// preview 3. 제어문 - if, match
fn main()
{	
	let score = 70;	

	// ❶ if 문
	if score >= 60		// 조건식에 () 가 필요 없다.
	{					// 실행할 문장이 한줄이라도 반드시 {}이 필요 하다.
	}
	else
	{
	}

	// ❷ match 문 - 아주 강력한 기능을 가진 제어문
	match score
	{
		0       => println!("zero"),	// (주의) ; 이 아닌 , 사용
		1 ..=39 => println!("fail"),	// 1 ~ 39
		40..=59 => println!("reexam"),	// 40 ~ 59
		_       => println!("pass"),	// 나머지의 경우
	}
}

