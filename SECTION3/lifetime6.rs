fn f1() -> &'static i32
{
//	let n = 10;
//	&n
	static N :i32 = 10;
	&N

}

fn main()
{
	let r = f1();
}

fn f2() -> &'static str
{
	"hello"
}
