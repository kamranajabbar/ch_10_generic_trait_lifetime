// Efficient code
pub fn run() {
    let integer = Option::Some(10);
    let float = Option::Some(10.99);

    println!("{:#?}, {:#?}", integer, float);
}

// None-efficient code
pub fn run_updated() {
    #[derive(Debug)]
    enum Option_i32 {
        Some(i32),
        None
    }

    #[derive(Debug)]
    enum Option_f64 {
        Some(f64),
        None
    }

    let integer = Option_i32::Some(3);
    let float = Option_f64::Some(3.3);

    println!("{:#?}, {:#?}", integer, float);
}