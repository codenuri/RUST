mod print_things {
    use std::fmt::Display;

    pub fn prints_one_thing<T: Display>(input: T) 
	{ 
		// Print anything that implements Display
        println!("{}", input)
    }
}

fn main()
{
//	crate::print_things::prints_one_thing(6);
	print_things::prints_one_thing(6);
}