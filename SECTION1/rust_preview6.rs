// preview 6. enum #1

// ❶ enum으로 새로운 타입 만들기 
enum ExamResult
{
	Pass,
	Fail,
}

fn main()
{
	// ❷ enum 변수 생성
	let result = ExamResult::Pass;
	
	// ❸ match 를 사용해서 enum 값에 따른 코드 작성
	match result
	{
		ExamResult::Pass => println!("pass"),	
		ExamResult::Fail => println!("fail"),
	}
}
