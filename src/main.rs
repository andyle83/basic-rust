use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for args in env::args().skip(1) {
        numbers.push(u64::from_str(&args).expect("Error parsing argument"));
    }

    if numbers.len() == 0 {
        eprint!("Unable: gcd NUMBER ....");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common division of {:?} is {}", numbers, d);

    number_type_println();
}

fn number_type_println() {
    let big_val = std::i32::MAX;

    println!("big_val = {}", big_val);

    // let x = big_val + 1;
    let big_val_plus_1 = big_val.wrapping_add(1);
    println!("big_val + 1 = {}", big_val_plus_1);

    let suffix_number_1 = 42u8;
    let suffix_number_2 = 1729isize;
    println!("Suffix numbers: {} and {}", suffix_number_1, suffix_number_2);

    let underscore_number = 4_294_295;
    let underscore_hex = 0xff_ff;
    println!("Underscore values : {} and {}", underscore_number, underscore_hex);

    // Return ASCII code of character
    let byte_literal_1 = b'A';
    let byte_literal_2 = 65u8;
    println!("Byte literal value: {} and {}", byte_literal_1, byte_literal_2);
}

fn gcd(mut n:u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m !=0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd( 2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
fn number_op() {
    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b101101u8.count_ones(), 4);
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!(-1.01f64.floor(), -1.);
}

#[test]
fn boolean_op() {
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

#[test]
fn character_op() {
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);

    match std::char::from_u32(0xca0) {
        Some(c) => println!("Character convert: {}", c),
        None => eprint!("Convert error")
    }
}