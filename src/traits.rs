pub fn run() {
    #[derive(Debug)]
    struct Superman {
        name: String,
    }

    #[derive(Debug)]
    struct Batman {
        name: String,
    }

    #[derive(Debug)]
    struct Hulk {
        name: String,
    }

    #[derive(Debug)]
    struct Spiderman {
        name: String,
    }


    pub trait Power {
        /* When we have defferent values for each superhere, so we have to used Custome impl*/
        // Custome impl
        //fn power_scroe(&self) -> u8; 
        
        

        /* When we have same value for two or more superheres, so we have to used Default impl */
        // Default impl
        fn power_score(&self) -> u8 {
            50
        }
    }

    impl Power for Superman {
        fn power_score(&self) ->u8 {
            100
        }
    }

    impl Power for Batman {
        fn power_score(&self) ->u8 {
            80
        }
    }

    /*
        When two or more superhere has defferent values in its implemetations

        impl Power for Hulk {
            fn power_score(&self) ->u8 {
                50
            }
        }

        impl Power for Spiderman {
            fn power_score(&self) ->u8 {
                50
            }
        }
    */

    /* When two or more superhere has same values in its implemetations, no needs to mention in curly brackets */
    impl Power for Hulk {}
    impl Power for Spiderman {}
    

    let my_superman = Superman {
        name : String::from("Superman"),
    };

    let my_batman = Batman {
        name : String::from("Batman"),
    };

    let my_hulk = Hulk {
        name : String::from("Hulk"),
    };

    let my_spiderman = Spiderman {
        name : String::from("Spiderman"),
    };

    // Print
    println!("{}", my_superman.power_score());
    println!("{}", my_batman.power_score());
    println!("{}", my_hulk.power_score());
    println!("{}", my_spiderman.power_score());
}