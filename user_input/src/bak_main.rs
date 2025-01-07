const WHTB: &str = "\x1b[1;37m";
const REDB: &str = "\x1b[1;31m";
const BLU: &str = "\x1b[0;34m";
const BLUB: &str = "\x1b[1;34m";
const YELB: &str = "\x1b[1;33m";
const NC: &str = "\x1b[0m";

fn pretty_print(point: &str, byte: &str) {
    println!("{BLU}{}{WHTB}{} - {BLU}{}{YELB}{}{NC}", &point[0..2], &point[2..], &byte[0..4], &byte[4..]);
}

fn check_len(point: u32) {
    //let s0 = format!("'{: ^8}'", char::from_u32(point).unwrap_or('�'));
    let s0 = format!("'{: ^8}'", format!("{:#04x}", point));
    print!("len(s0)={:02} : ", s0.len());
    let s1 = format!("{}", s0);
    print!("len(s1)={:02} : ", s1.len());
    println!("{}", s0);
}

fn main() {
    println!("\u{231A} \u{231B}");
    println!("{:#06X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:#06X}", 'H' as u32);
    println!("{:#06X}", '居' as u32);
    println!("{:#06X}", '美' as u32);
    println!("{:#06X}", 'い' as u32);
    println!("😂: {:#06X}", '😂' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    let uc = 0x1f602;
    // Because not all values of a u32 represent valid Unicode Scalar Values, you need to handle an error-case
    println!("{}", char::from_u32(uc).unwrap_or('�'));
    //println!("\u{1f602}");
    let uc = 0x11f602;
    println!("{}", char::from_u32(uc).unwrap_or('�'));


    let s1 = "1EF9";
    let s2 = "11010100";
    pretty_print(s1, s2);
    //println!("{BLU}{}{WHTB}{}{NC}", &s[0..2], &s[2..]);

    check_len(0x39);
    check_len(0x1d);
    check_len(0xf5);
    /*check_len(0x1f5);
    check_len(0x13f5);
    check_len(0x113f5);*/
}
