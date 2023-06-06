mod print_things 
{
    use std::fmt::{Display, Debug};

    #[derive(Debug)]
    pub struct Billy { // Billy is public
        name: String, // but name is private.
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self { // That means the user needs to use new to create a Billy. The user can only change the number of times_to_print
            Self {
                name: "Billy".to_string(), // We choose the name - the user can't
                times_to_print,
            }
        }

        pub fn print_billy(&self) { // This function prints a Billy
            for _ in 0..self.times_to_print {
                println!("{:?}", self.name);
            }
        }
    }

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input)
    }
}

fn main() {
    use crate::print_things::*; // Now we use *. This imports everything from print_things

    let my_billy = Billy::new(3);
    my_billy.print_billy();
}