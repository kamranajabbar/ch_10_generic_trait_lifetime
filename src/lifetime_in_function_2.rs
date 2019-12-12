pub fn run() {
    let s1 = String::from("Hello Kamran");
    {
        let s2 = String::from("Hello Pakistan");
        let result = longest(s1.as_str(), s2.as_str());

        println!("{}", result);
    }
}

fn longest <'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotations                     => &i32
// Reference with a given lifetime          => &'a i32
// Mutable reference with a given lifetime  => &'a mut i32