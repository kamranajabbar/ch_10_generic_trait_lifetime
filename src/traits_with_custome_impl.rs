pub fn run() {
    #[derive(Debug)]
    struct NewsArtical {
        author: String,
        content: String,
    }

    #[derive(Debug)]
    struct Tweet {
        username: String,
        content: String,
    }

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    impl Summary for NewsArtical {
        fn summarize(&self) -> String {
            format!("{} written by {}", self.content, self.author)
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} Tweeted {}", self.username, self.content)
        }
    }
    
    let tweet_1 = Tweet {
        username: String::from("Kamran Jabbar"),
        content: String::from("Pakistan Zindabad"),
    };

    let news_artical_1 = NewsArtical {
        author: String::from("Kamran Jabbar"),
        content: String::from("New Area of AIOT In Pakistan"),
    };

    println!("{}", tweet_1.summarize());
    println!("{}", news_artical_1.summarize());
}