pub fn run() {
    #[derive(Debug)]
    struct Tweet {
        username: String,
        content: String,
    }

    #[derive(Debug)]
    struct NewsArtical {
        author: String,
        content: String,
    }

    pub trait Summary {
        fn summarize(&self)->String;
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{} Posted this: {}", self.username, self.content)
        }
    }

    impl Summary for NewsArtical {
        fn summarize(&self) -> String {
            format!("{} Posted this: {}", self.author, self.content)
        }
    }

    fn notify<T: Summary> (item: T) -> String {
        format!("{}", item.summarize())
    }

    // let data = Tweet {
    //     username: String::from("Kamran Jabbar"),
    //     content: String::from("Pakistan Zindabad!"),
    // };

    let data = NewsArtical {
        author: String::from("Kamran Jabbar"),
        content: String::from("Pakistan Zindabad!"),
    };

    println!("{}", notify(data));
}