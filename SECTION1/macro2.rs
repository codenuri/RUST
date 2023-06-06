fn main()
{
	// ❶ cfg 매크로
//	if cfg!(target_os="linux")
	if cfg!(linux)
//	if cfg!(not(linux))
	{
		println!("linux");
	}
	else
	{
		println!("not linux");
	}	

	do_work();
}


// ❷ #[cfg( ... )] attribute
#[cfg(target_os="linux")]
fn do_work() { println!("linux"); }

#[cfg(not(target_os="linux"))]
fn do_work() { println!("not linux"); }




// ❸ OS 이름
// => windows, macos, ios, linux, android, freebsd, dragonfly, openbsd, netbsd
// => "The Rust Reference" 문서 참고
