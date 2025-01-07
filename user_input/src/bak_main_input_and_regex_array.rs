use regex::Regex;
use simple_user_input::get_input;


mod simple_user_input {
    use std::io;
    const YELB: &str = "\x1b[1;33m";
    const NC: &str = "\x1b[0m";
    pub fn get_input(prompt: &str) -> String {
        println!("{YELB}{prompt}{NC}");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}

fn main() {
    //let pattern = Regex::new(r"hello, (world|universe)!").unwrap();
    //let input = "hello, world!";
    //let pattern = Regex::new(r"^\[([1-9]|1[0-6]),([1-9]|1[0-6])\]").unwrap();
    //assert!(pattern.is_match("[9,12]"));
    //assert!(pattern.is_match("[10,2]"));
    //assert!(pattern.is_match("[12,12]"));
    //assert!(!pattern.is_match("[21,12]"));
    //let pattern = Regex::new(r"^([1-9]|1[0-6]), *([1-9]|1[0-6])$").unwrap();
    let pattern = Regex::new(r"^\\[x|X]([0-9]|[a-f]|[A-F]){2}(,([0-9]|[a-f]|[A-F]){2})*$").unwrap();

    let input: String = get_input("> Input your choice (Q: quit, S: show, P: print hex, R: reset, A: auto-parse, Row,Col: invert crossover):");
    //println!("User input>>: {}", input);

    match input.as_str() {
        "A" | "a" => println!("- User input: option 'A'"),
        "Q" | "q" => println!("- User input: option 'Q'"),
        "R" | "r" => println!("- User input: option 'R'"),
        "P" | "p" => println!("- User input: option 'P'"),
        "S" | "s" => println!("- User input: option 'S'"),
        _ => {
            if pattern.is_match(input.as_str()) {
                let input_str = &input[2..];
                let arr: Vec<&str> = input_str.split(",").collect();
                println!("> len = {}", arr.len());
                for item in arr {
                    println!("- {}", item);
                }
                /*let (x, y) = (
                    arr[0].trim().parse::<u16>().unwrap(),
                    arr[1].trim().parse::<u16>().unwrap(),
                );
                println!("- User input: [{x},{y}]");*/
            } else {
                println!("- Invalid input: \"{input}\"!");
            }
        }
    }
}
