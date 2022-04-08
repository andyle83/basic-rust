fn main() {
}

#[test]
fn byte_literal_check() {
    // byte literal - ASC characters only
    // emphasize value represent ASC character
    let a = b'a';
    let A = b'A';

    println!("{:?}", a);
    println!("{:?}", A);
    println!("{:?}", A);

    assert_eq!(a, 97);
    assert_eq!(A, 65);
}

#[test]
fn char_type_check() {
    // char - represent single Unicode character - 32 bits
    // String - sequence of UTF-8 bytes, not array of character

    // decimal
    assert_eq!('a' as i32, 97);
    assert_eq!('A' as i32, 65);

    // hex
    assert_eq!('a' as i32, 0x61);
    assert_eq!('A' as i32, 0x41);

    // oct
    assert_eq!('a' as i32, 0o0141);
    assert_eq!('A' as i32, 0o0101);
}