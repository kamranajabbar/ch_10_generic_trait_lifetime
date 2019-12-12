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
            format!("@{} Posted: {}", self.username, self.content)
        }
    }

    impl Summary for NewsArtical {
        fn summarize(&self) -> String {
            format!("{} Posted: {}", self.author, self.content)
        }
    }

    // This function is taking two differnt type parameters
    // fn notify(item1: impl Summary, item2: impl Summary) -> String {
    //     format!("{} and {}", item1.summarize(), item2.summarize())
    // }
    
    // This function is taking two same type parameters (used bound syntax)
    fn notify<T:Summary> (item1:T, item2:T) -> String {
        format!("{} and {}", item1.summarize(), item2.summarize())
    } 

    let tweet_1 = Tweet {
        username: String::from("Kamran Jabbar"),
        content: String::from("Pakistan Zindabad!"),
    };

    let tweet_2 = Tweet {
        username: String::from("Imran Jabbar"),
        content: String::from("Long Live Pakistan!"),
    };

    let news_artical_1 = NewsArtical {
        author: String::from("Zia Khan"),
        content: String::from("PIAIC is great opportunity for youth to boost Pakistan!"),
    };

    // With two differnt data types in parameters
    //println!("{}", notify(tweet_1, news_artical_1));
    
    // With two same data types in parameters
    println!("{}", notify(tweet_1, tweet_2));
}