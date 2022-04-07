use std::str::FromStr;
use std::env;
use std::fmt::format;
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

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

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!(" clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}

#[test]
fn ownership_op() {
    // ownership
    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!(label, "(0.625, 0.5)");
    }

    {
        let s  = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s;
        // let u = s;
    }

    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s.clone();
        let u = s.clone();
    }

    {
        let mut v = Vec::new();
        for i in 101 .. 106 {
            v.push(i.to_string());
        }

        // print vector
        println!("Vector {:?} ", v);

        // pull out random elements from the vector
        // move ownership
        //let third = v[2];
        //let fifth = v[4];
        let fifth = v.pop().unwrap();
        assert_eq!(fifth, "105");
        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        println!("Vector {:?} ", v);

        assert_eq!(v, vec!["101", "104", "103"]);

        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        assert_eq!(v, vec!["101", "104", "substitute"]);
    }
}

#[test]
fn move_op() {
    {
        struct Person { name: Option<String>, birth: i32 }
        let mut composers = Vec::new();
        composers.push(Person {
            name: Some("Anh Le".to_string()),
            birth: 1522
        });

        // birth is COPY type because it is a just a simple type - number
        let age = composers[0].birth;
        assert_eq!(age, 1522);
        assert_eq!(composers[0].name, Some("Anh Le".to_string()));
        assert_eq!((*composers)[0].name, Some("Anh Le".to_string()));

        //let name = composers[0].name;
        let name = composers[0].name.take();
        assert_eq!(name, Some("Anh Le".to_string()));
        assert_eq!(composers[0].name, None);
    }
}

#[test]
fn ref_op() {
    {
        let mut table = Table::new();
        table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
        assert_eq!(table["Caravaggio"][0], "The Musicians");
        show_without_ref(table);
        // moved
        // assert_eq!(table["Caravaggio"][0], "The Musicians");
    }

    {
        let mut table = Table::new();
        table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
        assert_eq!(table["Caravaggio"][0], "The Musicians");
        show_with_ref(&table);
        assert_eq!(table["Caravaggio"][0], "The Musicians");
    }

    {
        let mut table = Table::new();
        table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
        sort_with_mutable_ref(&mut table);
        println!("Table {:?} ", table);
        // assert_eq!(table["Caravaggio"][0], "The Musicians");
    }

    {
        let x = 10;
        let r = &x;
        assert!(*r == 10);
    }

    {
        let mut x = 10;
        let m = &mut x;
        assert!(*m == 10);
        *m = 20;
        assert!(*m == 20);
    }
}

fn show_without_ref(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist); for work in works {
            println!("  {}", work);
        }
    }
}

fn show_with_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist); for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_with_mutable_ref(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

#[test]
fn ref_compare() {
    let x = 10;
    let y = 10;

    assert!(x == y);

    let rx =  &x;
    let ry = &y;

    assert!(rx == ry);

    assert!(!std::ptr::eq(rx, ry));
}