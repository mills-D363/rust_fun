use regex::Regex;
use std::env;
use std::process;
use std::i64;

/* https://en.wikipedia.org/wiki/UTF-8 */
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

    let (mask_1, mask_2_1, mask_2_2, mask_3, mask_4, mask_5_1, mask_5_2, mask_6) = (0x04u8, 0x03u8, 0x30u8, 0x0fu8, 0x3cu8, 0x03u8, 0x30u8, 0x0fu8);
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
    let pattern = Regex::new(r"^([0-9]|[a-f]|[A-F]){2}$").unwrap();

    let args: Vec<String> = env::args().collect();
    let arg_num = args.len() - 1;
    if arg_num == 0 {
        eprintln!("*** No argument! ***");
        process::exit(2);
    }

    let mut byte_arr: [u8; 4] = [0, 0, 0, 0];
    let mut index = 0;
    for arg_str in &args[1..] {
        //println!("- {}", arg_str);
        if pattern.is_match(arg_str.as_str()) {
            let val_hex = i64::from_str_radix(&arg_str, 16).unwrap_or_else(|err| {
                eprintln!("Hex conversion failed: {err}");
                process::exit(1);
            });
            byte_arr[index] = val_hex as u8;
            println!("- {:#04x}", byte_arr[index]);
            index += 1;
        } else {
            eprintln!("*** Invalid u8 value in hex! ***");
            process::exit(2);
        }
    }

    match arg_num {
        1 => {
                match utf8_1byte_to_unicode(byte_arr[0]) {
                Err(why) => println!("Error: {}", why),
                Ok(point) => println!(">> Unicode Point: {:#06x} ('{}')", point, char::from_u32(point).unwrap_or('�')),
         }
        }
        2 => {
                match utf8_2bytes_to_unicode(byte_arr[0], byte_arr[1]) {
                Err(why) => println!("Error: {}", why),
                Ok(point) => println!(">> Unicode Point: {:#06x} ('{}')", point, char::from_u32(point).unwrap_or('�')),
         }
        }
        3 => {
                match utf8_3bytes_to_unicode(byte_arr[0], byte_arr[1], byte_arr[2]) {
                Err(why) => println!("Error: {}", why),
                Ok(point) => println!(">> Unicode Point: {:#06x} ('{}')", point, char::from_u32(point).unwrap_or('�')),
         }
        }
        4 => {
                match utf8_4bytes_to_unicode(byte_arr[0], byte_arr[1], byte_arr[2], byte_arr[3]) {
                Err(why) => println!("Error: {}", why),
                Ok(point) => println!(">> Unicode Point: {:#08x} ('{}')", point, char::from_u32(point).unwrap_or('�')),
         }
        }
        _ => eprintln!("*** Too many arguments! ***"),
    }
}
