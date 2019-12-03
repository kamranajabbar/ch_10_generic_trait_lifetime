#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    let integers = Point { x: 100, y: 101};
    let float = Point { x: 10.25, y: 20.26};

    println!("Integers {:?}", integers);
    println!("Floats {:?}", float);
}