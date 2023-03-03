use std::io::prelude::*;

fn write_file() -> std::io::Result<()> 
{
    let mut file = std::fs::File::create("sample.txt")?;
				
    file.write_all(b"hello ")?;
    file.write_all(b"world")?;
	
    Ok(())
}

fn main()
{
	let ret = write_file();
}