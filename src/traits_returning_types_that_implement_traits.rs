pub fn run() {
    #[derive(Debug)]
    struct Tweet {
        username: String,
        content: String,
    }

    pub trait Summary {
        fn summarize(&self)->String;
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{} Tweeted: {}", self.username, self.content)
        }
    }

    // Same datatype will be return that implement Summary
    fn create_summary(check: bool) -> impl Summary {
        if check {
            Tweet {
                username: String::from("Kamran Jabbar"),
                content: String::from("Pakistan Zindabad!")
            }
        } else {
            Tweet {
                username: String::from("Imran Jabbar"),
                content: String::from("Long Live Pakistan!")
            }
        }
    }

    let result = create_summary(false);
    println!("{}", result.summarize());
}