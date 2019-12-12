pub fn run() {
    let s1 = "abcd";
    let s2 = String::from("xyz");

    println!("{}", longest(s1, s2.as_str()));
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