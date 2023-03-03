// C++
int main()
{
	std::string s1 = "ABCD";

	std::string s2 = s1;			// 1. deep copy		
	std::string s2 = std::move(s1); // 3. move
}

// Rust
fn main()
{
	let s1 = String::from("ABCD");

//	let s2 = s1;			// 1. move	
	let s2 = s1.clone();	// 3. deep copy
}