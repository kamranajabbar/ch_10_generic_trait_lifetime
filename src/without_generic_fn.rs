pub fn run() {
    let nums = vec![70,20,30,40,50];
    let res = largest(&nums);

    println!("The largest number is {}", res);
}

fn largest(x: &[i32]) -> i32 {
    let mut largest = x[0];

    for &item in x {
        if item > largest {
            largest = item;
        }
    }

    largest
}