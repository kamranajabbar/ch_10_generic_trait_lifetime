#[derive(Debug)]
struct ImportantExpert <'a> {
    part: &'a str
}

pub fn run() {
    let novel = String::from("Today is not friday. Hi Kamran. its Sunday");
    let first_sentence = novel.split('.').next().expect("Failed to find the '.'");

    let i = ImportantExpert {
        part: first_sentence
    };

    println!("{:#?}", i);
}