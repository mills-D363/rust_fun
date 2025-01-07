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
        _ => {
            //println!("{BRNB}{: ^8} {:#04x}{NC}", char::from_u32(point_u32).unwrap_or('�'), point_u32),
            println!("{BRNB}{} {:#04x}{NC}", " ".repeat(8), point_u32);
            println!(
                "{BRNB}{: ^6}{NC}",
                format!("'{}'", char::from_u32(point_u32).unwrap_or('�'))
            );
        }
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
    let point_str = format!(
        "U+0{GRNB}{}{REDB}{}{PURB}{}{NC}",
        &str_p[1..2],
        &str_p[2..3],
        &str_p[3..]
    );
    let str_b1 = format!("{:08b}", byte1);
    let str_b2 = format!("{:08b}", byte2);
    let byte_str = format!(
        "{}{GRNB}{}{REDB}{}{NC}  {}{REDB}{}{PURB}{}{NC}",
        &str_b1[0..3],
        &str_b1[3..6],
        &str_b1[6..],
        &str_b2[0..2],
        &str_b2[2..4],
        &str_b2[4..]
    );

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str);
    match point_u32 {
        0x80..=0x9F => println!(
            "{BLUB}{: ^6}   {BRNB}{: <8}  {: <8}{NC}",
            "[ C1 ]",
            format!("{:#04x}", byte1),
            format!("{:#04x}", byte2)
        ),
        _ => {
            //println!("{BRNB}{: ^8} {: <8}  {: <8}{NC}", char::from_u32(point_u32).unwrap_or('�'), format!("{:#04x}", byte1), format!("{:#04x}", byte2)),
            println!(
                "{BRNB}{} {: <8}  {: <8}{NC}",
                " ".repeat(8),
                format!("{:#04x}", byte1),
                format!("{:#04x}", byte2)
            );
            println!(
                "{BRNB}{: ^6}{NC}",
                format!("'{}'", char::from_u32(point_u32).unwrap_or('�'))
            );
        }
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
    let point_str = format!(
        "U+{BLUB}{}{GRNB}{}{REDB}{}{PURB}{}{NC}",
        &str_p[0..1],
        &str_p[1..2],
        &str_p[2..3],
        &str_p[3..]
    );
    let str_b1 = format!("{:08b}", byte1);
    let str_b2 = format!("{:08b}", byte2);
    let str_b3 = format!("{:08b}", byte3);
    let byte_str = format!(
        "{}{BLUB}{}{NC}  {}{GRNB}{}{REDB}{}{NC}  {}{REDB}{}{PURB}{}{NC}",
        &str_b1[0..4],
        &str_b1[4..],
        &str_b2[0..2],
        &str_b2[2..6],
        &str_b2[6..],
        &str_b3[0..2],
        &str_b2[2..4],
        &str_b3[4..]
    );

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str);
    //println!("{BRNB}{: ^8} {: <8}  {: <8}  {: <8}{NC}", char::from_u32(point_u32).unwrap_or('�'), format!("{:#04x}", byte1), format!("{:#04x}", byte2), format!("{:#04x}", byte3));
    println!(
        "{BRNB}{} {: <8}  {: <8}  {: <8}{NC}",
        " ".repeat(8),
        format!("{:#04x}", byte1),
        format!("{:#04x}", byte2),
        format!("{:#04x}", byte3)
    );
    println!(
        "{BRNB}{: ^6}{NC}",
        format!("'{}'", char::from_u32(point_u32).unwrap_or('�'))
    );
}

fn print_conversion_4bytes() {
    println!("{}", "—".repeat(73));
    println!("{WHTB}First code point  Last code point  Byte 1    Byte 2    Byte 3    Byte 4{NC}");
    println!("        U+{DRED}0{YELB}1{BLUB}0{GRNB}0{REDB}0{PURB}0{NC}         U+{DRED}1{YELB}0{BLUB}F{GRNB}F{REDB}F{PURB}F{NC}  11110{DRED}x{YELB}xx{NC}  10{YELB}xx{BLUB}xxxx{NC}  10{GRNB}xxxx{REDB}xx{NC}  10{REDB}xx{PURB}xxxx{NC}");
    println!("{}", "—".repeat(73));
}

fn pretty_print_4bytes(point_u32: u32, byte1: u8, byte2: u8, byte3: u8, byte4: u8) {
    let str_p = format!("{:06x}", point_u32);
    let point_str = format!(
        "U+{DRED}{}{YELB}{}{BLUB}{}{GRNB}{}{REDB}{}{PURB}{}{NC}",
        &str_p[0..1],
        &str_p[1..2],
        &str_p[2..3],
        &str_p[3..4],
        &str_p[4..5],
        &str_p[5..]
    );
    let str_b1 = format!("{:08b}", byte1);
    let str_b2 = format!("{:08b}", byte2);
    let str_b3 = format!("{:08b}", byte3);
    let str_b4 = format!("{:08b}", byte4);
    let byte_str = format!("{}{DRED}{}{YELB}{}{NC}  {}{YELB}{}{BLUB}{}{NC}  {}{GRNB}{}{REDB}{}{NC}  {}{REDB}{}{PURB}{}{NC}", &str_b1[0..5], &str_b1[5..6], &str_b1[6..], &str_b2[0..2], &str_b2[2..4], &str_b2[4..], &str_b3[0..2], &str_b2[2..6], &str_b3[6..], &str_b4[0..2], &str_b4[2..4], &str_b4[4..]);

    println!("{}{BRNB} ↔ {NC}{}", point_str, byte_str);
    //println!("{BRNB}{: ^8}   {: <8}  {: <8}  {: <8}  {: <8}{NC}", char::from_u32(point_u32).unwrap_or('�'), format!("{:#04x}", byte1), format!("{:#04x}", byte2), format!("{:#04x}", byte3), format!("{:#04x}", byte4));
    println!(
        "{BRNB}{} {: <8}  {: <8}  {: <8}  {: <8}{NC}",
        " ".repeat(10),
        format!("{:#04x}", byte1),
        format!("{:#04x}", byte2),
        format!("{:#04x}", byte3),
        format!("{:#04x}", byte4)
    );
    println!(
        "{BRNB}{: ^8}{NC}",
        format!("'{}'", char::from_u32(point_u32).unwrap_or('�'))
    );
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

fn convert_unicode_to_utf8(point_u32: u32) {
    match point_u32 {
        0x0000..=0x007F => {
            print_conversion_1byte();
            pretty_print_1byte(point_u32);
        }
        0x0080..=0x07FF => {
            print_conversion_2bytes();
            let (b1, b2) = unicode_to_utf8_2bytes(point_u32);
            pretty_print_2bytes(point_u32, b1, b2);
        }
        0x0800..=0xFFFF => {
            print_conversion_3bytes();
            let (b1, b2, b3) = unicode_to_utf8_3bytes(point_u32);
            pretty_print_3bytes(point_u32, b1, b2, b3);
        }
        0x010000..=0x10FFFF => {
            print_conversion_4bytes();
            let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(point_u32);
            pretty_print_4bytes(point_u32, b1, b2, b3, b4);
        }
        _ => println!("{REDB}Out of range of unicode points!{NC}"),
    }
}

fn utf8_1byte_to_unicode(byte1: u8) -> Result<u32, String> {
    //let mut point: u32;

    let (header_1, mark_h1) = (0b10000000u8, 0x00u8);

    if byte1 & header_1 != mark_h1 {
        return Err(String::from("Invalid byte 1!"));
    }

    Ok(byte1 as u32)
}

fn utf8_2bytes_to_unicode(byte1: u8, byte2: u8) -> Result<u32, String> {
    let mut point: u32 = 0;

    let (header_1, headers) = (0b11100000u8, 0b11000000u8);
    let (mark_h1, mark) = (0b11000000u8, 0b10000000u8);

    if byte1 & header_1 == mark_h1 {
        if byte2 & headers != mark {
            return Err(String::from("Invalid byte 2!"));
        }
    } else {
        return Err(String::from("Invalid byte 1!"));
    }

    let (mask_1, mask_2_1, mask_2_2, mask_3) = (0x1cu8, 0x03u8, 0x30u8, 0x0fu8);
    point = point | (byte2 & mask_3) as u32;
    point = point | (((byte2 & mask_2_2) | ((byte1 & mask_2_1) << 6)) as u32);
    point = point | (((byte1 & mask_1) as u32) << 6);

    Ok(point)
}

fn utf8_3bytes_to_unicode(byte1: u8, byte2: u8, byte3: u8) -> Result<u32, String> {
    let mut point: u32 = 0;

    let (header_1, headers) = (0b11110000u8, 0b11000000u8);
    let (mark_h1, mark) = (0b11100000u8, 0b10000000u8);

    if byte1 & header_1 == mark_h1 {
        if byte2 & headers == mark {
            if byte3 & headers != mark {
                return Err(String::from("Invalid byte 3!"));
            }
        } else {
            return Err(String::from("Invalid byte 2!"));
        }
    } else {
        return Err(String::from("Invalid byte 1!"));
    }

    let (mask_1, mask_2, mask_3_1, mask_3_2, mask_4) = (0x0fu8, 0x3cu8, 0x03u8, 0x30u8, 0x0fu8);
    point = point | ((byte3 & mask_4) as u32);
    point = point | (((byte3 & mask_3_2) | ((byte2 & mask_3_1) << 6)) as u32);
    point = point | (((byte2 & mask_2) as u32) << 6);
    point = point | (((byte1 & mask_1) as u32) << 12);

    Ok(point)
}

fn utf8_4bytes_to_unicode(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> Result<u32, String> {
    let mut point: u32 = 0;

    let (header_1, headers) = (0b11111000u8, 0b11000000u8);
    let (mark_h1, mark) = (0b11110000u8, 0b10000000u8);

    if byte1 & header_1 == mark_h1 {
        if byte2 & headers == mark {
            if byte3 & headers == mark {
                if byte4 & headers != mark {
                    return Err(String::from("Invalid byte 4!"));
                }
            } else {
                return Err(String::from("Invalid byte 3!"));
            }
        } else {
            return Err(String::from("Invalid byte 2!"));
        }
    } else {
        return Err(String::from("Invalid byte 1!"));
    }

    let (mask_1, mask_2_1, mask_2_2, mask_3, mask_4, mask_5_1, mask_5_2, mask_6) = (
        0x04u8, 0x03u8, 0x30u8, 0x0fu8, 0x3cu8, 0x03u8, 0x30u8, 0x0fu8,
    );
    point = point | ((byte4 & mask_6) as u32);
    point = point | (((byte4 & mask_5_2) | ((byte3 & mask_5_1) << 6)) as u32);
    point = point | (((byte3 & mask_4) as u32) << 6);
    point = point | (((byte2 & mask_3) as u32) << 12);
    point = point | ((((byte2 & mask_2_2) | ((byte1 & mask_2_1) << 6)) as u32) << 12);
    point = point | ((((byte1 & mask_1) >> 2) as u32) << 20);

    Ok(point)
}

fn main() {
    print_conversion_all();
    let ptn_char = Regex::new(r"^>.$").unwrap();
    let pattern = Regex::new(r"^[U|u]\+([0-9]|[a-f]|[A-F]){1,6}$").unwrap();
    let ptn_utf8 =
        Regex::new(r"^\\[x|X]([0-9]|[a-f]|[A-F]){2}(,([0-9]|[a-f]|[A-F]){2})*$").unwrap();

    loop {
        let input: String =
            get_input("\nPlease type the character (e.g.: \x1b[1;37m>\x1b[0m😂), unicode point (e.g.: \x1b[1;37mU+\x1b[0m13f5) or utf-8 bytes (e.g.: \x1b[1;37m\\X\x1b[0me1,8f,b5) ['Q' to quit]:");

        match input.as_str() {
            "Q" | "q" => {
                break;
            }
            _ => {
                if ptn_char.is_match(input.as_str()) {
                    let ch: char = input[1..].chars().next().unwrap();
                    let v_u32: u32 = ch as u32;
                    println!(">> Unicode Point: U+{YELB}{:04x}{NC}", v_u32);
                    convert_unicode_to_utf8(v_u32);
                } else {
                    if pattern.is_match(input.as_str()) {
                        let input_point = &input[2..];
                        let v_u32: u32 = i64::from_str_radix(input_point, 16).unwrap() as u32;
                        println!(">> Unicode Point: U+{YELB}{:04x}{NC}", v_u32);
                        convert_unicode_to_utf8(v_u32);
                    } else {
                        if ptn_utf8.is_match(input.as_str()) {
                            let input_str = &input[2..];
                            let arr: Vec<&str> = input_str.split(",").collect();
                            let utf8_bytes = arr.len();
                            if utf8_bytes <= 4 {
                                let mut byte_arr: [u8; 4] = [0, 0, 0, 0];
                                let mut index = 0;
                                for item in arr {
                                    let val_hex = i64::from_str_radix(&item, 16).unwrap();
                                    byte_arr[index] = val_hex as u8;
                                    //println!("- {:#04x}", byte_arr[index]);
                                    index += 1;
                                }
                                match utf8_bytes {
                                    1 => match utf8_1byte_to_unicode(byte_arr[0]) {
                                        Err(why) => println!("Error: {}", why),
                                        Ok(point) => println!(
                                            ">> Unicode Point: U+{YELB}{:04x}{NC}  ('{WHTB}{}{NC}')",
                                            point,
                                            char::from_u32(point).unwrap_or('�')
                                        ),
                                    },
                                    2 => match utf8_2bytes_to_unicode(byte_arr[0], byte_arr[1]) {
                                        Err(why) => println!("Error: {}", why),
                                        Ok(point) => println!(
                                            ">> Unicode Point: U+{YELB}{:04x}{NC}  ('{WHTB}{}{NC}')",
                                            point,
                                            char::from_u32(point).unwrap_or('�')
                                        ),
                                    },
                                    3 => {
                                        match utf8_3bytes_to_unicode(
                                            byte_arr[0],
                                            byte_arr[1],
                                            byte_arr[2],
                                        ) {
                                            Err(why) => println!("Error: {}", why),
                                            Ok(point) => println!(
                                                ">> Unicode Point: U+{YELB}{:04x}{NC}  ('{WHTB}{}{NC}')",
                                                point,
                                                char::from_u32(point).unwrap_or('�')
                                            ),
                                        }
                                    }
                                    4 => {
                                        match utf8_4bytes_to_unicode(
                                            byte_arr[0],
                                            byte_arr[1],
                                            byte_arr[2],
                                            byte_arr[3],
                                        ) {
                                            Err(why) => println!("Error: {}", why),
                                            Ok(point) => println!(
                                                ">> Unicode Point: U+{YELB}{:06x}{NC}  ('{WHTB}{}{NC}')",
                                                point,
                                                char::from_u32(point).unwrap_or('�')
                                            ),
                                        }
                                    }
                                    _ => eprintln!("{REDB}*** Invalid utf-8 sequence! ***{NC}"),
                                }
                            } else {
                                eprintln!("{REDB}*** Invalid utf-8 sequence! ***{NC}");
                            }
                        } else {
                            eprintln!("{REDB}*** Invalid input: \"{input}\"! ***{NC}");
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_unicode_to_utf8_2bytes() {
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
    fn cases_unicode_to_utf8_3bytes() {
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
    fn cases_unicode_to_utf8_4bytes() {
        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x10000);
        assert_eq!((b1, b2, b3, b4), (0xf0, 0x90, 0x80, 0x80));

        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x1577c);
        assert_eq!((b1, b2, b3, b4), (0xf0, 0x95, 0x9d, 0xbc));

        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x1089aa);
        assert_eq!((b1, b2, b3, b4), (0xf4, 0x88, 0xa6, 0xaa));

        let (b1, b2, b3, b4) = unicode_to_utf8_4bytes(0x10ffff);
        assert_eq!((b1, b2, b3, b4), (0xf4, 0x8f, 0xbf, 0xbf));
    }

    #[test]
    fn cases_utf8_to_unicode_2bytes() {
        assert_eq!(utf8_2bytes_to_unicode(0xc2, 0x80), Ok(0x80));
        assert_eq!(utf8_2bytes_to_unicode(0xc2, 0x9a), Ok(0x9a));
        assert_eq!(utf8_2bytes_to_unicode(0xcf, 0xb7), Ok(0x3f7));
        assert_eq!(utf8_2bytes_to_unicode(0xc2, 0x89), Ok(0x89));
        assert_eq!(utf8_2bytes_to_unicode(0xdf, 0xbf), Ok(0x7ff));
    }

    #[test]
    fn cases_utf8_to_unicode_3bytes() {
        assert_eq!(utf8_3bytes_to_unicode(0xe0, 0xa0, 0x80), Ok(0x800));
        assert_eq!(utf8_3bytes_to_unicode(0xe0, 0xa2, 0xae), Ok(0x8ae));
        assert_eq!(utf8_3bytes_to_unicode(0xe3, 0xad, 0x8c), Ok(0x3b4c));
        assert_eq!(utf8_3bytes_to_unicode(0xef, 0xbf, 0xbf), Ok(0xffff));
        assert_eq!(utf8_3bytes_to_unicode(0xe1, 0xcf, 0xb8), Err(String::from("Invalid byte 2!")));
    }

    #[test]
    fn cases_utf8_to_unicode_4bytes() {
        assert_eq!(utf8_4bytes_to_unicode(0xf0, 0x90, 0x80, 0x80), Ok(0x10000));
        assert_eq!(utf8_4bytes_to_unicode(0xf0, 0x95, 0x9d, 0xbc), Ok(0x1577c));
        assert_eq!(utf8_4bytes_to_unicode(0xf4, 0x88, 0xa6, 0xaa), Ok(0x1089aa));
        assert_eq!(utf8_4bytes_to_unicode(0xf4, 0x8f, 0xbf, 0xbf), Ok(0x10ffff));
        assert_eq!(utf8_4bytes_to_unicode(0xe4, 0x8f, 0xbf, 0xbf), Err(String::from("Invalid byte 1!")));
    }
}
