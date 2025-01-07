use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let char_vec: Vec<char> = args[1].chars().collect();
    for cv in char_vec {
        println!("{}: {:#06X}", cv, cv as u32);
    }
}
