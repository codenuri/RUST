#[must_use]
fn db_backup() -> i32
{
	-1   // 실패라고 가정
}

fn db_remove()
{
	println!("DB REMOVED")
}
   
fn main()
{
	db_backup();

	db_remove();
}