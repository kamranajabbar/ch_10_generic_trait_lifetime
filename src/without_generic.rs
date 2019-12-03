pub fn run() {
    let v = vec![10,20,30,40,50];
    let mut largest = v[0];

    for item in v {
        if item > largest {
            largest = item;
        }
    }

    println!("The largest number is {}", largest);
}