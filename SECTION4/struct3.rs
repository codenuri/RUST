struct Point
{
	x : i32,
	y : i32,
}

fn main()
{
	let x = 5;
	let y = 3;

	let pt1 = Point{x:10, y:20};
	let pt2 = Point{y:20, x:10}; // ok
//	let pt3 = Point{x:10};		 // error
	let pt4 = Point{x, y};		 // ok
	let pt5 = Point{x:30, ..pt1};// ok
	let pt6 = Point{y:30, ..pt1};// ok

	let mut pt7 = Point{x:10, y:20};
//	pt1.x = 30; // error
	pt7.x = 30; // ok


	let x1 = pt1.x;
	let y1 = pt1.x;

	let Point{x:x2, y:y2} = pt1;

	let Point{x, y} = pt2;
	
	println!("{}, {}", x2, y2); // 10, 20
	println!("{}, {}", x, y);	// 10, 20
}