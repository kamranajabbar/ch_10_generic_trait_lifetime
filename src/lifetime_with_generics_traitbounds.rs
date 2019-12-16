use std::fmt::Display;

pub fn run() {
    fn longest_with_announcement <'a, T: Display> (x: &'a str, y: &'a str, ann: T) -> &'a str {
        println!("{}", ann);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!("{} \n", longest_with_announcement("Hello Kamran", "Hi Kamran", 2020));
    println!("{} \n", longest_with_announcement("Hello Kamran", "Hi Kamran", 20.10));
    println!("{} \n", longest_with_announcement("Hello Kamran", "Hi Kamran", "PIAIC"));
}