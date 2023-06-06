fn db_backup()
{

	let test = DropTest;

	println!("DB BACKUP");

//	std::process::exit(-1);	
	panic!("fail db_backup");
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







struct DropTest;

impl Drop for DropTest {
    fn drop(&mut self) {
        println!("**== drop called ==**");
    }
}
