#[cfg(test)]
mod traits {
    use std::fs::File;
    use std::io::Write;
    use std::cmp::Ord;

    // Trait example
    // &mut dyn Write is a trait object
    // fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    // T: Write is call type parameter
    fn say_hello<T: Write>(out: &mut T) -> std::io::Result<()> {
        out.write_all(b"Aa");
        out.flush()
    }

    #[test]
    fn test_say_hello() {
        let mut local_file = File::create("hello.txt").expect("Unable to create file");
        let mut bytes: Vec<u8> = vec![];

        // convert reference to trait object
        say_hello(&mut bytes);
        say_hello(&mut local_file);

        println!("Value in buf {:?}", bytes);
    }


    // Generic
    fn get_min<T: Ord>(value1: T, value2: T) -> T {
        if value1 < value2 {
            value1
        } else {
            value2
        }
    }

    #[test]
    fn test_get_min() {
        let value1 = 100;
        let value2 = 200;

        assert_eq!(get_min(value1, value2), value1);
        assert_eq!(get_min(value2, value1), value1);

        let str1 = String::from("hello1");
        let str2 = String::from("hello2");

        assert_eq!(get_min(&str1, &str2), &str1);
        assert_eq!(get_min(&str2, &str1), &str1);
    }

    // trait object
    #[test]
    fn test_trait_object() {
        use std::io::Write;
        // reference to a trait type
        let mut buf: Vec<u8> = vec![];

        // writer is a trait object - Or it is a reference to a trait type
        let writer: &mut dyn Write = &mut buf;

        // writer is a fat pointer, that point to
        // - value
        // - table represent value type (its trait)
        (*writer).write_all(b"This is a message");
        (*writer).flush();

        println!("Data in buf: {:?}", buf);
        assert_eq!(buf, b"This is a message");
    }

    trait TaskHandling {
        fn get_requirement(&self);
        fn analyze_requirement(&self);
        fn implement_requirement(&self);
        fn review(&self);
        fn test(&self);
        fn release(&self) {
            println!("This is a default implement");
        }
    }
    
    struct Task {
        id: u32,
        title: String,
        description: String,
        report_to: String,
        assign_to: String,
        status: String
    }

    impl TaskHandling for Task {
        fn get_requirement(&self) {
            println!("Get Requirement {}", self.id);
        }

        fn analyze_requirement(&self) {
            println!("Analyze Requirement");
            println!("Title {}", self.title);
            println!("Description {}", self.description);
        }

        fn implement_requirement(&self) {
            println!("Implement Requirement");
        }

        fn review(&self) {
            println!("Review");
        }

        fn test(&self) {
            println!("Test");
        }

        // fn release(&self) {
        //     println!("Release");
        // }
    }

}