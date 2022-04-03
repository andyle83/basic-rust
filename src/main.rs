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
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('8'.to_digit(10), Some(8));

    match std::char::from_u32(0xca0) {
        Some(c) => println!("Character convert: {}", c),
        None => eprint!("Convert error")
    }
}

#[test]
fn tuple_op() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn array_op() {
    let lazy_caterer: [u32; 6] = [1,2,3,4,5,6];
    let taxonomy = ["Tuan", "Anh", "Le"];

    assert_eq!(lazy_caterer[3], 4);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);
}

#[test]
fn vector_op() {
    let mut v = vec![2,3,5,7];
    assert_eq!(v.iter().fold(2, | a, b| a * b), 420);

    v.push(2);
    assert_eq!(v.iter().fold(2, | a, b| a * b), 840);

    v.pop();
    assert_eq!(v.iter().fold(2, | a, b| a * b), 420);
}

fn print_slide(n: &[f64]) {
    for elt in n {
        print!("{} ", elt);
    }
    println!()
}

#[test]
fn slide_op() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] =  [0.0, -0.707, -1.0, -0.707];

    print_slide(&v);
    print_slide(&a);

    println!("Slide 1 {:?} ", &v[0..2]);
    println!("Slide 2 {:?}", &a[2..]);
}

#[test]
fn string_op() {
    let mut noodles = "noodles".to_string();
    // slice string
    let oodles = &noodles[1..];
    // string literal has a pre-allocated memory
    let poodles = "ಠ_ಠ";

    assert_eq!(poodles.len(), 7);
    assert_eq!(poodles.chars().count(), 3);

    // Not able
    //poodles.push('\n');
    noodles.push('s');
}