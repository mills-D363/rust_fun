use regex::Regex;
use simple_user_input::get_input;
use std::i64;

const DRED: &str = "\x1b[38;2;204;0;0m";
const WHTB: &str = "\x1b[1;37m";
const REDB: &str = "\x1b[1;31m";
const BLU: &str = "\x1b[0;34m";
const BLUB: &str = "\x1b[1;34m";
const PURB: &str = "\x1b[1;35m";
const GRNB: &str = "\x1b[1;32m";
const YELB: &str = "\x1b[1;33m";
const BRNB: &str = "\x1b[1;33m";
const NC: &str = "\x1b[0m";
//const BLK: &str = "\x1b[0;30m";
//const GRY: &str = "\x1b[2;37m";
//const LCYB: &str = "\x1b[1;36m";

fn print_conversion_1byte() {
    println!("{}", "—".repeat(73));
    println!("{WHTB}First code point  Last code point  Byte 1    Byte 2    Byte 3    Byte 4{NC}");
    println!("          U+00{REDB}0{PURB}0{NC}           U+00{REDB}7{PURB}F{NC}  0{REDB}xxx{PURB}xxxx{NC}");
    println!("{}", "—".repeat(73));
}

fn pretty_print_1byte(point_u32: u32) {
    let str_p = format!("{:04x}", point_u32);
    let point_str = format!("U+00{REDB}{}{PURB}{}{NC}", &str_p[2..3], &str_p[3..]);
    let str_b = format!("{:08b}", point_u32);
    let byte_str = format!("0{REDB}{}{PURB}{}{NC}", &str_b[1..4], &str_b[4..]);

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str); 
    match point_u32 {
        0x00..=0x1F => println!("{BLUB}{: ^6}   {BRNB}{:#04x}{NC}", "[ C0 ]", point_u32),
        0x7F => println!("{BLUB}{: ^6}   {BRNB}{:#04x}{NC}", "DEL", point_u32),
        _ => println!("{BRNB}{: ^8} {:#04x}{NC}", char::from_u32(point_u32).unwrap_or('�'), point_u32),
    }
}

fn print_conversion_2bytes() {
    println!("{}", "—".repeat(73));
    println!("{WHTB}First code point  Last code point  Byte 1    Byte 2    Byte 3    Byte 4{NC}");
    println!("          U+0{GRNB}0{REDB}8{PURB}0{NC}           U+0{GRNB}7{REDB}F{PURB}F{NC}  110{GRNB}xxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("{}", "—".repeat(73));
}

fn pretty_print_2bytes(point_u32: u32, byte1: u8, byte2: u8) {
    let str_p = format!("{:04x}", point_u32);
    let point_str = format!("U+0{GRNB}{}{REDB}{}{PURB}{}{NC}", &str_p[1..2], &str_p[2..3], &str_p[3..]);
    let str_b1 = format!("{:08b}", byte1);
    let str_b2 = format!("{:08b}", byte2);
    let byte_str = format!("{}{GRNB}{}{REDB}{}{NC}  {}{REDB}{}{PURB}{}{NC}", &str_b1[0..3], &str_b1[3..6], &str_b1[6..], &str_b2[0..2], &str_b2[2..4], &str_b2[4..]);

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str); 
    match point_u32 {
        0x80..=0x9F => println!("{BLUB}{: ^6}   {BRNB}{: <8}  {: <8}{NC}", "[ C1 ]", format!("{:#04x}", byte1), format!("{:#04x}", byte2)),
        _ => println!("{BRNB}{: ^8} {: <8}  {: <8}{NC}", char::from_u32(point_u32).unwrap_or('�'), format!("{:#04x}", byte1), format!("{:#04x}", byte2)),
    }
}

fn print_conversion_3bytes() {
    println!("{}", "—".repeat(73));
    println!("{WHTB}First code point  Last code point  Byte 1    Byte 2    Byte 3    Byte 4{NC}");
    println!("          U+{BLUB}0{GRNB}8{REDB}0{PURB}0{NC}           U+{BLUB}F{GRNB}F{REDB}F{PURB}F{NC}  1110{BLUB}xxxx{NC}  10{GRNB}xxxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("{}", "—".repeat(73));
}

fn pretty_print_3bytes(point_u32: u32, byte1: u8, byte2: u8, byte3: u8) {
    let str_p = format!("{:04x}", point_u32);
    let point_str = format!("U+{BLUB}{}{GRNB}{}{REDB}{}{PURB}{}{NC}", &str_p[0..1], &str_p[1..2], &str_p[2..3], &str_p[3..]);
    let str_b1 = format!("{:08b}", byte1);
    let str_b2 = format!("{:08b}", byte2);
    let str_b3 = format!("{:08b}", byte3);
    let byte_str = format!("{}{BLUB}{}{NC}  {}{GRNB}{}{REDB}{}{NC}  {}{REDB}{}{PURB}{}{NC}", &str_b1[0..4], &str_b1[4..], &str_b2[0..2], &str_b2[2..6], &str_b2[6..], &str_b3[0..2], &str_b2[2..4], &str_b3[4..]);

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str); 
    println!("{BRNB}{: ^8} {: <8}  {: <8}  {: <8}{NC}", char::from_u32(point_u32).unwrap_or('�'), format!("{:#04x}", byte1), format!("{:#04x}", byte2), format!("{:#04x}", byte3));
}

fn print_conversion_4bytes() {
    println!("{}", "—".repeat(73));
    println!("{WHTB}First code point  Last code point  Byte 1    Byte 2    Byte 3    Byte 4{NC}");
    println!("        U+{DRED}0{YELB}1{BLUB}0{GRNB}0{REDB}0{PURB}0{NC}         U+{DRED}1{YELB}0{BLUB}F{GRNB}F{REDB}F{PURB}F{NC}  11110{DRED}x{YELB}xx{NC}  10{YELB}xx{BLUB}xxxx{NC}  10{GRNB}xxxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("{}", "—".repeat(73));
}

fn pretty_print_4bytes(point_u32: u32, byte1: u8, byte2: u8, byte3: u8, byte4: u8) {
    let str_p = format!("{:06x}", point_u32);
    let point_str = format!("U+{DRED}{}{YELB}{}{BLUB}{}{GRNB}{}{REDB}{}{PURB}{}{NC}", &str_p[0..1], &str_p[1..2], &str_p[2..3], &str_p[3..4], &str_p[4..5], &str_p[5..]);
    let str_b1 = format!("{:08b}", byte1);
    let str_b2 = format!("{:08b}", byte2);
    let str_b3 = format!("{:08b}", byte3);
    let str_b4 = format!("{:08b}", byte4);
    let byte_str = format!("{}{DRED}{}{YELB}{}{NC}  {}{YELB}{}{BLUB}{}{NC}  {}{GRNB}{}{REDB}{}{NC}  {}{REDB}{}{PURB}{}{NC}", &str_b1[0..5], &str_b1[5..6], &str_b1[6..], &str_b2[0..2], &str_b2[2..4], &str_b2[4..], &str_b3[0..2], &str_b2[2..6], &str_b3[6..], &str_b4[0..2], &str_b4[2..4], &str_b4[4..]);

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str); 
    println!("{BRNB}{: ^8}   {: <8}  {: <8}  {: <8}  {: <8}{NC}", char::from_u32(point_u32).unwrap_or('�'), format!("{:#04x}", byte1), format!("{:#04x}", byte2), format!("{:#04x}", byte3), format!("{:#04x}", byte4));
}

/* https://en.wikipedia.org/wiki/UTF-8 */
fn print_conversion_all() {
    println!("{BRNB}Code point ↔ UTF-8 conversion{BLU} (https://en.wikipedia.org/wiki/UTF-8){NC}");
    println!("{}", "—".repeat(73));
    println!("{WHTB}First code point  Last code point  Byte 1    Byte 2    Byte 3    Byte 4{NC}");
    println!("          U+00{REDB}0{PURB}0{NC}           U+00{REDB}7{PURB}F{NC}  0{REDB}xxx{PURB}xxxx{NC}");
    println!("          U+0{GRNB}0{REDB}8{PURB}0{NC}           U+0{GRNB}7{REDB}F{PURB}F{NC}  110{GRNB}xxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("          U+{BLUB}0{GRNB}8{REDB}0{PURB}0{NC}           U+{BLUB}F{GRNB}F{REDB}F{PURB}F{NC}  1110{BLUB}xxxx{NC}  10{GRNB}xxxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("        U+{DRED}0{YELB}1{BLUB}0{GRNB}0{REDB}0{PURB}0{NC}         U+{DRED}1{YELB}0{BLUB}F{GRNB}F{REDB}F{PURB}F{NC}  11110{DRED}x{YELB}xx{NC}  10{YELB}xx{BLUB}xxxx{NC}  10{GRNB}xxxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("{}", "—".repeat(73));
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

fn unicode_to_utf8_2bytes(point_u32: u32) -> (u8, u8) {
    let (mask_1, mask_2, mask_3) = (0x0f00u32, 0x00f0u32, 0x000fu32);
    let (base_b1, base_b2) = (0xc0u8, 0x80u8);

    let b1_p1: u8 = ((mask_1 & point_u32) >> 6) as u8;
    let b1_p2: u8 = ((mask_2 & point_u32) >> 6) as u8;

    let b2_p1: u8 = ((mask_2 & point_u32) as u8) & 0x30;
    let b2_p2: u8 = (mask_3 & point_u32) as u8;

    let b1: u8 = base_b1 | b1_p1 | b1_p2;
    let b2: u8 = base_b2 | b2_p1 | b2_p2;

    (b1, b2)
}

fn unicode_to_utf8_3bytes(point_u32: u32) -> (u8, u8, u8) {
    let (mask_1, mask_2, mask_3, mask_4) = (0xf000u32, 0x0f00u32, 0x00f0u32, 0x000fu32);
    let (base_b1, base_b2, base_b3) = (0xe0u8, 0x80u8, 0x80u8);

    let b1_p1: u8 = ((mask_1 & point_u32) >> 12) as u8;

    let b2_p1: u8 = ((mask_2 & point_u32) >> 6) as u8;
    let b2_p2: u8 = ((mask_3 & point_u32) >> 6) as u8;

    let b3_p1: u8 = ((mask_3 & point_u32) as u8) & 0x30;
    let b3_p2: u8 = (mask_4 & point_u32) as u8;

    let b1: u8 = base_b1 | b1_p1;
    let b2: u8 = base_b2 | b2_p1 | b2_p2;
    let b3: u8 = base_b3 | b3_p1 | b3_p2;

    (b1, b2, b3)
}

fn unicode_to_utf8_4bytes(point_u32: u32) -> (u8, u8, u8, u8) {
    let (mask_1, mask_2, mask_3, mask_4, mask_5, mask_6) = (
        0x300000u32,
        0x030000u32,
        0xf000u32,
        0x0f00u32,
        0x00f0u32,
        0x000fu32,
    );
    let (base_b1, base_b2, base_b3, base_b4) = (0xf0u8, 0x80u8, 0x80u8, 0x80u8);

    let b1_p1: u8 = ((mask_1 & point_u32) >> 18) as u8;
    let b1_p2: u8 = ((mask_2 & point_u32) >> 18) as u8;

    let b2_p1: u8 = (((mask_2 & point_u32) >> 12) as u8) & 0x30;
    let b2_p2: u8 = ((mask_3 & point_u32) >> 12) as u8;

    let b3_p1: u8 = ((mask_4 & point_u32) >> 6) as u8;
    let b3_p2: u8 = ((mask_5 & point_u32) >> 6) as u8;

    let b4_p1: u8 = ((mask_5 & point_u32) as u8) & 0x30;
    let b4_p2: u8 = (mask_6 & point_u32) as u8;

    let b1: u8 = base_b1 | b1_p1 | b1_p2;
    let b2: u8 = base_b2 | b2_p1 | b2_p2;
    let b3: u8 = base_b3 | b3_p1 | b3_p2;
    let b4: u8 = base_b4 | b4_p1 | b4_p2;

    (b1, b2, b3, b4)
}

fn main() {
    print_conversion_all();
    let pattern = Regex::new(r"^([0-9]|[a-f]|[A-F]){1,6}$").unwrap();

    loop {
        let input: String =
            get_input("\nPlease type the unicode point (e.g.: 13f5) ['Q' to quit]:");

        match input.as_str() {
            "Q" | "q" => {
                break;
            }
            _ => {
                if pattern.is_match(input.as_str()) {
                    let val_hex = i64::from_str_radix(&input, 16).unwrap();
                    println!(">> {:#08x}", val_hex);
                    match val_hex {
                        0x0000..=0x007F => {
                            print_conversion_1byte();
                            let v_u32: u32 = val_hex as u32;
                            pretty_print_1byte(v_u32);
                        }
                        0x0080..=0x07FF => {
                            print_conversion_2bytes();
                            let v_u32: u32 = val_hex as u32;
                            let (b1, b2) = unicode_to_utf8_2bytes(v_u32);
                            pretty_print_2bytes(v_u32, b1, b2);
                        }
                        0x0800..=0xFFFF => {
                            print_conversion_3bytes();
                            let v_u32: u32 = val_hex as u32;
                            let (b1, b2, b3) = unicode_to_utf8_3bytes(v_u32);
                            pretty_print_3bytes(v_u32, b1, b2, b3);
                        }
                        0x010000..=0x10FFFF => {
                            print_conversion_4bytes();
                            let v_u32: u32 = val_hex as u32;
                            let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(v_u32);
                            pretty_print_4bytes(v_u32, b1, b2, b3, b4);
                        }
                        _ => println!("{REDB}Out of range of unicode points!{NC}"),
                    }
                } else {
                    println!("{REDB}*** Invalid input: \"{input}\"! ***{NC}");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_2bytes() {
        let (b1, b2) = unicode_to_utf8_2bytes(0x80);
        assert_eq!((b1, b2), (0xc2, 0x80));

        let (b1, b2) = unicode_to_utf8_2bytes(0x9a);
        assert_eq!((b1, b2), (0xc2, 0x9a));

        let (b1, b2) = unicode_to_utf8_2bytes(0x3f7);
        assert_eq!((b1, b2), (0xcf, 0xb7));

        let (b1, b2) = unicode_to_utf8_2bytes(0x89);
        assert_eq!((b1, b2), (0xc2, 0x89));

        let (b1, b2) = unicode_to_utf8_2bytes(0x7ff);
        assert_eq!((b1, b2), (0xdf, 0xbf));
    }

    #[test]
    fn cases_3bytes() {
        let (b1, b2, b3) = unicode_to_utf8_3bytes(0x800);
        assert_eq!((b1, b2, b3), (0xe0, 0xa0, 0x80));

        let (b1, b2, b3) = unicode_to_utf8_3bytes(0x8ae);
        assert_eq!((b1, b2, b3), (0xe0, 0xa2, 0xae));

        let (b1, b2, b3) = unicode_to_utf8_3bytes(0x3b4c);
        assert_eq!((b1, b2, b3), (0xe3, 0xad, 0x8c));

        let (b1, b2, b3) = unicode_to_utf8_3bytes(0xffff);
        assert_eq!((b1, b2, b3), (0xef, 0xbf, 0xbf));
    }

    #[test]
    fn cases_4bytes() {
        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x10000);
        assert_eq!((b1, b2, b3, b4), (0xf0, 0x90, 0x80, 0x80));

        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x1577c);
        assert_eq!((b1, b2, b3, b4), (0xf0, 0x95, 0x9d, 0xbc));

        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x1089aa);
        assert_eq!((b1, b2, b3, b4), (0xf4, 0x88, 0xa6, 0xaa));

        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x10ffff);
        assert_eq!((b1, b2, b3, b4), (0xf4, 0x8f, 0xbf, 0xbf));
    }
}
