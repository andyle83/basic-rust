mod r#type;

fn main() {
}

#[test]
fn array_type_check() {
    // an array living in heap memory
    let array_number = [1, 2, 3, 4, 5, 6];
    let array_string = ["le", "tuan", "anh"];

    assert_eq!(array_number.len(), 6);
    assert_eq!(array_string.len(), 3);
    assert_eq!(array_number[0], 1);
    assert_eq!(array_string[0], "le");

    let mut unsorted_numbers = [1, 3, 2, 5, 4];
    unsorted_numbers.sort();
    assert_eq!(unsorted_numbers[1], 2);
    assert_eq!(unsorted_numbers[2], 3);
    assert_eq!(unsorted_numbers[3], 4);
}

#[test]
fn vector_type_check() {
    let mut v = vec!['a', 'b', 'c', 'd'];
    v.push('e');
    v.push('f');

    assert_eq!(v.pop(), Some('f'));
    assert_eq!(v.pop(), Some('e'));

    assert_eq!(v.len(), 4);

    for val in v.iter() {
        println!("Got {}", val);
    }

    // another way to create vector
    let n: Vec<i32> = (1..3).collect();
    let f = n.iter().fold(0, | s1, s2 | s1 + s2 );
    assert_eq!(f, 3);

    // a vector (pointer) container:
    // 1. pointer to heap memory
    // 2. capacity
    // 3. actual contain
    let mut c = Vec::with_capacity(2);
    assert_eq!(c.len(), 0);
    assert_eq!(c.capacity(), 2);

    c.push(1);
    c.push(2);
    c.push(3);

    assert_eq!(c.len(), 3);
    assert!(!(c.capacity() == 2));
}

#[test]
fn slice_type_check() {
    // slice is another pointer
    // point to a region of an array or vector
    // 1. we also have STRING slice, which is a bit different
    // 2. this a fat-pointer, including
    // - point to first element's memory
    // - number of element in slice

    // vector
    let v = vec![1, 2, 3];
    // array
    let a = [4, 5, 6];

    // slice to vector
    let sv : &[i32] = &v;
    let sa : &[i32] = &a;

    // iterate
    for e in sv {
        println!("{}", e);
    }

    for e in sa {
        println!("{}", e);
    }

    // get few elements by slice
    println!("{:?}",&v[0..2]);
}

#[test]
fn string_check_types() {
    // By https://doc.rust-lang.org/, there are two types of string
    // 1. String literal
    // 2. &str: string slice

    // 1. String literal
    // - vector of bytes Vec<u8> - UTF8 sequence
    // - heap allocation
    // - because it's vector of bytes, its fat-pointer include
    // + pointer to address
    // + capacity
    // + actual numbers of element (depending)

    // represent by double quotes
    let first_speech = "Good morning Melbourne \n".to_string();
    println!("{}", first_speech);

    // represent with raw
    let second_speech = r"Rust compiler path - C:\Program Files\Rust".to_string();
    println!("{}", second_speech);

    // represent by byte string - look like a slice of an array
    let third_speech = b"GET";
    let fourth_speed = &[b'G', b'E', b'T'];
    assert_eq!(third_speech, fourth_speed);

    // 2. string slice
    // Reference (borrow reference) to UTF-8 owned by something else
    // Fat pointer, contain:
    // - Pointer
    // - Its length
    let fifth_speech = &first_speech[0..4];
    let sixth_speech = "Good";
    assert_eq!(fifth_speech, sixth_speech);
    // len - number of byte
    assert_eq!(sixth_speech.len(), 4);
}