/* 
    Elision lifetime Rule 1:
    First rule implemented on "input lifetime parameters (s: &'a str)"

    i.e:
    fn first_word <'a> (s: &'a str) -> &'a str {

    }

    Elision lifetime Rule 2,3:
    First rule implemented on "output lifetime parameters "-> &'a str" "

    i.e:
    fn first_word <'a> (s: &'a str) -> &'a str {

    }

    NOTE:
    As a programmer, we do not to follow, but complier follow these rules. As these rules applies on function bodies and implementation bodies (impl).


    ---------------------------- Rules ---------------------------- 

    Elision lifetime Rule 1:
    The first rule is that each parameter that is a reference gets its own lifetime parameter.

    EXAMPLES:
    A function with one parameter gets one lifetime paramter.
    fn foo <'a> (x: &'a i32);

    A function with two parameters gets two lifetime paramters.
    fn foo <'a, 'b> (x: &'a i32, y: &'b i32);

    A function with three parameters gets three lifetime paramters.
    fn foo <'a, 'b, 'c> (x: &'a i32, y: &'b i32, z: &'c i32);

    and so on...


    Elision lifetime Rule 2:
    The second rule says, if there is exactly one input lifetime parameter, that lifetime parameter is assigned to output lifetime parameters.
    
    EXAMPLE:
    A function with one input lifetime parameter is assigned to all output lifetime paramter.
    fn foo <'a> (x: &'a i32) -> &'a i32;


    Elision lifetime Rule 3:
    The third rule says, if there are multiple input lifetime parameters, but one of theme is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
    
    NOTE:
    This rule makes methods much nicer to read and write because fewer symbols are necessary.
    
    EXAMPLES:
    fn foo <'a, 'b> (&'a self, x: &'a i32) -> &'a i32;
    
    OR

    fn foo <'a, 'b> (&mut'a self, x: &'a i32) -> &mut'a i32;
*/


pub fn run() {
    // Without lifetime annotation
    fn first_words(s: &str) -> &str {
        let bytes = s.as_bytes();
        println!("{:#?}", bytes);

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i] 
            }
        }
        &s[..]
    }

    // Function with lifetime annotation
    // As compiler will infer lifetime paramters automatically
    // Rule 1 and 2 are implementated by compiler
    // Rule 3 is not applicable as "self" parameter is not using here.

    // fn first_words <'a> (s: &'a str) -> &'a str {
    //     let bytes = s.as_bytes();
    //     println!("{:#?}", bytes);

    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return &s[0..i] 
    //         }
    //     }
    //     &s[..]
    // }  

    println!("{}", first_words("Hello Kamran"));
}