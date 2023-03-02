fn main()
{	
	// ❶ binary, octal, dec, hex 
	let n1 = 10;	// 10진수
	let n2 = 0b10;	// 2진수
	let n3 = 0o10;	// 8진수
	let n4 = 0x10;	// 16진수

	// ❷ literal suffix
	let v1       = 10;	// i32 type
	let v2 : u32 = 10;	// u32 type
	
	let v3 = 10u32;		// u32 type
	let v4 = 10_u32;	// u32 type
	let v5 = 3.4;		// f64 type
	let v6 = 3.4f32;	// f32 type
	let v7 = 0x10u8;	// u8  type 

	// ❸ digit separater
	let n6 = 1_000_000;
}