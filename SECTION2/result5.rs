use std::io::prelude::*;

fn write_file() -> std::io::Result<()> 
{
    let mut file = match std::fs::File::create("sample.txt") 
					{
						Err(e) => return Err(e),
						Ok(f)  => f,
					};

    if let Err(e) = file.write_all(b"hello ") 
	{
        return Err(e)
    }
    if let Err(e) = file.write_all(b"world") 
	{
        return Err(e)
    }

    Ok(())
}

fn main()
{
	let ret = write_file();
}