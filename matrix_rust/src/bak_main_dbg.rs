//use std::mem::size_of;

const MAXNUMS: usize = 16;
//const BUFFLEN: usize = size_of::<u16>() * MAXNUMS;
const BUFFLEN: usize = 32;

//const RED: &str = "\x1b[2;31m";
const REDB: &str = "\x1b[1;31m";
//const GRN: &str = "\x1b[0;32m";
const BLU: &str = "\x1b[0;34m";
//const BRN: &str = "\x1b[0;33m";
//const GRY: &str = "\x1b[2;37m";
//const YEL: &str = "\x1b[1;33m";
//const LCY: &str = "\x1b[1;36m";
const NC: &str = "\x1b[0m";

//static mut act_row1: u16 = 0;

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

fn display_u16_to_binary(n: u16) {
    let mut i: u16 = 0x8000;
    while i > 0 {
        if n & i > 0 {
            print!("  {}1", REDB);
        } else {
            print!("  {}0", NC);
        }
        i = i >> 1;
    }
    println!("{}", NC);
}

fn display_matrix(p_matrix: &[u16]) {
    for i in 0..MAXNUMS {
        display_u16_to_binary(p_matrix[i]);
    }
}

// i_col: 1 ~ 16
fn invert_node(p_row: &mut u16, i_col: usize) {
    assert_ne!(i_col,0);
    let mut i_tmp: u16 = 0x8000;
    let i_col = i_col -1;
    i_tmp = i_tmp >> i_col;
    
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

    let i_rowv: u16 = 0;
    let mut bn: u16 = 0;
    let mut cnt: u16 = 0;
    let mut min_steps: u16 = 65535;

    let mut i_traverse_v_in_1st_row: u16 = 1;

    let mut GSTEPS: u16 = 0;
    
    init_test_matrix(&mut i_matrix_arr);
    display_matrix(&i_matrix_arr);

    println!("\n{}\n", "=".repeat(50));

    // Backup current latest matrix
    copy_matrix(&i_matrix_arr, &mut i_matrix_bak);
    //display_matrix(&i_matrix_bak);

    i_traverse_v_in_1st_row = 0xc000;

    GSTEPS = 0;

    // Traverse all the possible attempts (1 ~ 65535) for the 1st row first, then process
    // the following rows one by one till the last row is empty (succeed) or not (fail)
    cnt = 0;
    bn = 0x8000;
    while bn > 0 {
        cnt += 1;
        if (i_traverse_v_in_1st_row & bn) > 0 {
            invert_crossover(&mut i_matrix_arr, 1, cnt.into());
            println!(" * Updated [1, {cnt}]:  {:#018b} *", i_matrix_arr[0]);
            GSTEPS += 1;
        }
        bn = bn >> 1;
    }

    println!("-- Row[1] = {:#018b}, Steps: {GSTEPS} --", i_matrix_arr[0]);
    // After the 1st row is processed, process each row one by one for every '1' node by
    // inverting the crossover centered at the node just beneath it.
    for i_rowv in 1..MAXNUMS {
        cnt = 0;
        bn = 0x8000;
        while bn > 0 {
            cnt += 1;
            if (i_matrix_arr[i_rowv - 1] & bn) > 0 {
                invert_crossover(&mut i_matrix_arr, i_rowv + 1, cnt.into());
                GSTEPS += 1;
            }
            bn = bn >> 1;
        }
        println!("-> Row[{}] = {:#018b}, Steps: {GSTEPS} <-", i_rowv, i_matrix_arr[i_rowv-1]);
    }
    if i_matrix_arr[MAXNUMS - 1] == 0 {
        println!("{BLU}>>> Row1: {i_traverse_v_in_1st_row}, Steps: {GSTEPS} <<<\n");
    } else {
        println!("FAILED: Last row  = {:#018b}", i_matrix_arr[MAXNUMS - 1]);
    }

    if GSTEPS < min_steps {
        min_steps = GSTEPS;
    }

    println!("\nmin_steps = {min_steps} \n");
    display_matrix(&i_matrix_arr);
}
