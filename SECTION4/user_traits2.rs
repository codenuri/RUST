struct MyType;

trait MyTrait
{
	type Output;

	fn method1(&self) {}
	fn method1(&self) -> Self::Output;
	fn afunction() {}
}

impl MyTrait for MyType
{
	type Output = i32;

	fn method1(&self) -> Self::Output
	{
		10
	}
}

fn main()
{
	
}