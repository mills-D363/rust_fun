use simple_user_input::get_input;
use regex::Regex;

fn main() {
    //let pattern = Regex::new(r"hello, (world|universe)!").unwrap();
    //let input = "hello, world!";
    //let pattern = Regex::new(r"^\[([1-9]|1[0-6]),([1-9]|1[0-6])\]").unwrap();
    //assert!(pattern.is_match("[9,12]"));
    //assert!(pattern.is_match("[10,2]"));
    //assert!(pattern.is_match("[12,12]"));
    //assert!(!pattern.is_match("[21,12]"));
    let pattern = Regex::new(r"^([1-9]|1[0-6]),([1-9]|1[0-6])$").unwrap();

    let input: String = get_input("Please type your input: ");
    //println!("User input>>: {}", input);

    match input.as_str() {
        "A" => println!("- User input: option 'A'"),
        "Q" => println!("- User input: option 'Q'"),
        "" => println!("- User input: none"),
        _ => { 
               if pattern.is_match(input.as_str()) {
                   println!("- User input: \"{input}\"");
                } else {
                   println!("- Invalid input: \"{input}\"!");
                }
            }
    }

    println!("{}", "=".repeat(50));
    let collection: Vec<&str> = input.split(",").collect();
    //dbg!(collection);
    for part in collection {
        println!("  > '{}'", part)
   }
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}
