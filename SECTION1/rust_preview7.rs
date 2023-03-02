// preview 7. enum #2   

// ❶ 각 항목(variants) 에는 연관된 값을 보관 가능.
enum ExamResult
{
	Pass(i32),	// Pass 와 연관된 i32 타입의 값을 한개 보관 가능.
	Fail,
}

fn main()
{
	let result1 = ExamResult::Pass(85);
	let result2 = ExamResult::Fail;

	// ❷ match 를 사용한 patter matching 으로 연관된 값 얻기
	match result1
	{
		ExamResult::Pass(score) => println!("pass : {score}"),
		_        				=> println!("fail"),
	}
}
