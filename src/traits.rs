#[cfg(test)]
mod traits {
    use std::fs::File;
    use std::io::Write;
    use std::io::Result;

    // Using generic function with trait
    fn say_hello<W: Write>(out: &mut W) -> Result<()> {
        out.write_all(b"Aa");
        out.flush()
    }

    #[test]
    fn test_say_hello() {
        let mut local_file = File::create("hello.txt");
        let mut bytes: Vec<u8> = vec![];

        say_hello(&mut bytes);
        // say_hello(&mut local_file);

        println!("Value in buf {:?}", bytes);
    }

}