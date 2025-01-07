//use std::mem::size_of;
use regex::Regex;
use simple_user_input::get_input;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

mod simple_user_input {
    const YELB: &str = "\x1b[1;33m";
    const NC: &str = "\x1b[0m";
    use std::io;
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

const DATFILEPATH: &str = "./matrix_data.sav";
const MAXNUMS: usize = 16;
//const BUFFLEN: usize = size_of::<u16>() * MAXNUMS;
//const BUFFLEN: usize = 32;

const WHT: &str = "\x1b[38;2;255;255;255m";
const REDB: &str = "\x1b[1;31m";
const BLU: &str = "\x1b[0;34m";
const GRNB: &str = "\x1b[1;32m";
const NC: &str = "\x1b[0m";
//const YELB: &str = "\x1b[1;33m";
//const BRN: &str = "\x1b[0;33m";
//const GRY: &str = "\x1b[2;37m";
//const LCYB: &str = "\x1b[1;36m";

fn read_matrix_from_file(p_matrix: &mut [u16]) {
    let mut f = File::open(DATFILEPATH).expect("open file failed");
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data).expect("read failed");

    let u8buflen = data.len();

    let mut idx = 0;
    let mut cnt = 0;
    while idx < u8buflen - 1 {
        p_matrix[cnt] = (data[idx + 1] as u16) << 8 | data[idx] as u16;
        cnt += 1;
        idx += 2;
    }
    println!("Successfully read matrix data from {}", DATFILEPATH);
}

fn write_matrix_to_file(p_matrix: &[u16]) {
    let path = Path::new(DATFILEPATH);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).expect("creation failed");

    let buf_matrix_le: &[u8] = unsafe { p_matrix.align_to::<u8>().1 };

    // Write bytes to `file`, returns `io::Result<()>`
    file.write_all(buf_matrix_le).expect("write failed");
    println!("Successfully write current matrix data to {}", DATFILEPATH);
}

fn init_test_matrix(p_matrix: &mut [u16]) {
    let mut i = 0;
    p_matrix[i] = 0x2802;
    i += 1;
    p_matrix[i] = 0x9803;
    i += 1;
    p_matrix[i] = 0xe60c;
    i += 1;
    p_matrix[i] = 0x1404;
    i += 1;
    p_matrix[i] = 0xd800;
    i += 1;
    p_matrix[i] = 0x5200;
    i += 1;
    p_matrix[i] = 0x0780;
    i += 1;
    p_matrix[i] = 0x0140;
    i += 1;
    p_matrix[i] = 0x0600;
    i += 1;
    p_matrix[i] = 0x0260;
    i += 1;
    p_matrix[i] = 0x0040;
    i += 1;
    p_matrix[i] = 0x0002;
    i += 1;
    p_matrix[i] = 0x2005;
    i += 1;
    p_matrix[i] = 0x3005;
    i += 1;
    p_matrix[i] = 0xc001;
    i += 1;
    p_matrix[i] = 0x4004;
}

fn reset_matrix(p_matrix: &mut [u16]) {
    p_matrix[..].iter_mut().for_each(|x| *x = 0x0000);
}

fn copy_matrix(ptr_source: &[u16], ptr_dest: &mut [u16]) {
    assert_eq!(ptr_source.len(), ptr_dest.len());
    ptr_dest[..].copy_from_slice(&ptr_source[..]);
}

fn display_u16_to_binary(p_matrix: &[u16], index: usize) {
    let mut bitval: u16 = 0x8000;
    print!("{WHT}{:#2}", index + 1);
    while bitval > 0 {
        if p_matrix[index] & bitval > 0 {
            print!("  {}1", REDB);
        } else {
            print!("  {}0", NC);
        }
        bitval >>= 1;
    }
    println!("{}", NC);
}

fn display_matrix(p_matrix: &[u16]) {
    //println!("{WHT}    1     3     5     7     9  10    12    14    16{NC}");
      println!("{WHT}       2     4     6     8     10    12    14    16{NC}");
    for i in 0..MAXNUMS {
        display_u16_to_binary(p_matrix, i);
    }
}

fn output_matrix_in_hex(p_matrix: &[u16]) {
    for i in 0..MAXNUMS {
        println!("{BLU}  Row[{:02}]: {:#06x}", i + 1, p_matrix[i]);
    }
}

fn is_empty_matrix(p_matrix: &[u16]) -> bool {
    let mut ret = true;
    for i in 0..MAXNUMS {
        if p_matrix[i] > 0 {
            ret = false;
            break;
        }
    }
    ret
}

// i_col: 1 ~ 16
fn invert_node(p_row: &mut u16, i_col: usize) {
    assert_ne!(i_col, 0);
    let mut i_tmp: u16 = 0x8000;
    let i_col = i_col - 1;
    i_tmp >>= i_col;

    if (*p_row & i_tmp) > 0 {
        *p_row = *p_row & (0xffff ^ i_tmp);
    } else {
        *p_row = *p_row | i_tmp;
    }
}

// i_row & i_col: 1 ~ 16
fn invert_crossover(p_matrix: &mut [u16], i_row: usize, i_col: usize) {
    let i_row = i_row - 1;
    invert_node(&mut p_matrix[i_row], i_col);

    if i_row >= 1 {
        invert_node(&mut p_matrix[i_row - 1], i_col);
    }
    if i_row <= 14 {
        invert_node(&mut p_matrix[i_row + 1], i_col);
    }
    if i_col >= 2 {
        invert_node(&mut p_matrix[i_row], i_col - 1);
    }
    if i_col <= 15 {
        invert_node(&mut p_matrix[i_row], i_col + 1);
    }
}

fn main() {
    let mut i_matrix_arr: [u16; MAXNUMS] = [0; MAXNUMS];
    let mut i_matrix_bak: [u16; MAXNUMS] = [0; MAXNUMS];

    let mut bn: u16;
    let mut cnt: u16;

    let mut gsteps: u16;
    let mut min_steps: u16;
    let mut act_row1: u16;
    let mut i_traverse_v_in_1st_row: u16;

    let pattern = Regex::new(r"^([1-9]|1[0-6]),([1-9]|1[0-6])$").unwrap();

    init_test_matrix(&mut i_matrix_arr);

    println!("{}", "=".repeat(52));
    display_matrix(&i_matrix_arr);
    println!("{}", "=".repeat(52));

    loop {
        let input: String = get_input("\n> Input your choice (Q: quit, S: show, P: print hex, R: reset, W: write to file, L: Load from file, A: auto-parse, Row,Col: invert crossover):");
        //println!("User input>>: {}", input);

        match input.as_str() {
            "Q" | "q" => {
                println!("- User input: option 'Q'");
                break;
            }
            "R" | "r" => {
                println!("- User input: option 'R'");
                reset_matrix(&mut i_matrix_arr);
            }
            "P" | "p" => {
                println!("- User input: option 'P'");
                output_matrix_in_hex(&i_matrix_arr);
            }
            "S" | "s" => {
                println!("- User input: option 'S'");
                display_matrix(&i_matrix_arr);
            }
            "W" | "w" => {
                println!("- User input: option 'W'");
                if Path::new(DATFILEPATH).exists() {
                    let promptstr = format!("* Are you sure to overwrite the existing matrix data file '{DATFILEPATH}'? (Y/n)");
                    let dbl_chk: String = get_input(&promptstr);
                    match dbl_chk.as_str() {
                        "Y" | "y" => {
                            write_matrix_to_file(&i_matrix_arr);
                        }
                        _ => {
                            println!("Aborted!");
                        }
                    }
                } else {
                    write_matrix_to_file(&i_matrix_arr);
                }
            }
            "L" | "l" => {
                println!("- User input: option 'L'");
                if Path::new(DATFILEPATH).exists() {
                    let promptstr = format!("* Are you sure to load matrix data from file '{DATFILEPATH}'? (Y/n)");
                    let dbl_chk: String = get_input(&promptstr);
                    match dbl_chk.as_str() {
                        "Y" | "y" => {
                            read_matrix_from_file(&mut i_matrix_arr);
                        }
                        _ => {
                            println!("Aborted!");
                        }
                    }
                } else {
                    println!("{REDB}*** Matrix data file '{DATFILEPATH}' not found! ***{NC}");
                }
            }
            "A" | "a" => {
                println!("- User input: option 'A'");
                if is_empty_matrix(&i_matrix_arr) {
                    println!("{REDB}*** Empty matrix, Abort! ***{NC}");
                } else {
                    min_steps = 65535;
                    act_row1 = 0;
                    i_traverse_v_in_1st_row = 0;
                    // Backup current latest matrix
                    copy_matrix(&i_matrix_arr, &mut i_matrix_bak);
                    loop {
                        gsteps = 0;

                        // Traverse all the possible attempts (1 ~ 65535) for the 1st row first, then process
                        // the following rows one by one till the last row is empty (succeed) or not (fail)
                        cnt = 0;
                        bn = 0x8000;
                        while bn > 0 {
                            cnt += 1;
                            if (i_traverse_v_in_1st_row & bn) > 0 {
                                invert_crossover(&mut i_matrix_arr, 1, cnt.into());
                                gsteps += 1;
                            }
                            bn >>= 1;
                        }

                        // After the 1st row is processed, process each row one by one for every '1' node by
                        // inverting the crossover centered at the node just beneath it.
                        for i_rowv in 1..MAXNUMS {
                            cnt = 0;
                            bn = 0x8000;
                            while bn > 0 {
                                cnt += 1;
                                if (i_matrix_arr[i_rowv - 1] & bn) > 0 {
                                    invert_crossover(&mut i_matrix_arr, i_rowv + 1, cnt.into());
                                    gsteps += 1;
                                }
                                bn >>= 1;
                            }
                        }
                        if i_matrix_arr[MAXNUMS - 1] == 0 {
                            println!(
                                "{BLU}>>> Row1: {:#018b}, Steps: {gsteps} <<<",
                                i_traverse_v_in_1st_row
                            );
                        }

                        // Restore_matrix from backup
                        copy_matrix(&i_matrix_bak, &mut i_matrix_arr);

                        if gsteps < min_steps {
                            act_row1 = i_traverse_v_in_1st_row;
                            min_steps = gsteps;
                        }

                        if i_traverse_v_in_1st_row < 65535 {
                            i_traverse_v_in_1st_row += 1;
                        } else {
                            break;
                        }
                    }

                    println!(
                        "\n{GRNB}Min_Steps = {min_steps}, Row1: {:#018b}{NC}",
                        act_row1
                    );
                }
            }
            _ => {
                if pattern.is_match(input.as_str()) {
                    let arr: Vec<&str> = input.split(",").collect();
                    let (x, y) = (
                        arr[0].parse::<usize>().unwrap(),
                        arr[1].parse::<usize>().unwrap(),
                    );
                    //println!("- User input: \"{input}\" (i.e.: [{x},{y}])");
                    println!("- User input: [{x},{y}]");
                    invert_crossover(&mut i_matrix_arr, x, y);
                    display_matrix(&i_matrix_arr);
                } else {
                    println!("- Invalid input: \"{input}\"!");
                }
            }
        }
    }
}
