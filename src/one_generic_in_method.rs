#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

pub fn run() {
    let p = Point { x: 100, y: 101};
    let f = Point { x: 10.11, y: 20.22};

    //For x points of p and f struct
    println!("p.x = {}", p.x());
    println!("f.y = {}", f.x());

    //For y points of p and f struct
    println!("p.x = {}", p.y());
    println!("f.y = {}", f.y());
}