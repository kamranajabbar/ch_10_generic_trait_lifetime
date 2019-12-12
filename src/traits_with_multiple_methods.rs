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
        // Method 1
        // No defauld implementation
        fn summarize_author(&self) -> String;
        
        // Method 2
        // Defauld implementation
        fn summarize(&self) -> String {
            format!("{}", self.summarize_author())
        }
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("{}", self.username)
        }
    }
    
    let tweet_1 = Tweet {
        username: String::from("Kamran Jabbar"),
        content: String::from("Pakistan Zindabad"),
    };

    println!("{}", tweet_1.summarize());
}