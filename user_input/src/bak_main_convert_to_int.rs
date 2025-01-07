fn main() {
    let my_string = "27".to_string();  // `parse()` works with `&str` and `String`!
    let my_str= "29";  // `parse()` works with `&str` and `String`!
    let my_int1 = my_string.parse::<i32>().unwrap();
    let my_int2 = my_str.parse::<usize>().unwrap();
    let my_int3: u16 = my_str.parse().unwrap();
    println!("{my_int1}, {my_int2}, {my_int3}");
}

