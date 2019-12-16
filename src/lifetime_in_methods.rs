#[derive(Debug)]
struct ImportantExpert <'a> {
    part: &'a str
}

impl <'a> ImportantExpert <'a> {
    fn new(&self) -> i32 {
        3
    }

    fn announce_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}

pub fn run() {
    let novel = String::from("Today is not friday. Hi Kamran. its Sunday");
    let first_sentence = novel.split('.').next().expect("Failed to find the '.'");

    let i = ImportantExpert {
        part: first_sentence
    };

    println!("{:#?}", i.new());
    println!("{:#?}", i.announce_return_part("Hello KJ!"));
}