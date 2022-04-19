#[cfg(test)]
mod traits {
    use std::fs::File;
    use std::io::Write;
    use std::io::Result;

    fn say_hello<W: Write>(out: &mut W) -> Result<()> {
        out.write(b"Aa");
        out.flush()
    }

    #[test]
    fn test_say_hello() {
        // let mut local_file = File::create("hello.txt");
        let mut buf: Vec<u8> = vec![];
        say_hello(&mut buf);

        println!("Value in buf {:?}", buf);
    }

}