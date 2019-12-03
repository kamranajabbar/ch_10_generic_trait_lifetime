#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn run() {
    let integers = Point { x: 100, y: 101};
    let float = Point { x: 10.25, y: 20.26};
    let integer_float = Point { x: 10, y: 20.26};

    println!("Integers {:?}", integers);
    println!("Floats {:?}", float);
    println!("Integers and Floats {:?}", integer_float);
}