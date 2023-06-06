struct MyType1
{
	x : i32,
	y : i32,
}
struct MyType2<T>
{
	x : T,
	y : T,
}
fn main()
{
	let t : MyType1      = MyType1{x:10, y:20};

	let t : MyType2<i32> = MyType2::<i32>{x:10, y:20};
}