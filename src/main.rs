fn main() {
}

#[test]
fn byte_literal_check() {
    // byte literal - ASC characters only
    let a = b'a';
    let A = b'A';

    println!("{:?}", a);
    println!("{:?}", A);

    assert_eq!(a, 97);
    assert_eq!(A, 65);
}