#[cfg(test)]
mod traits {
    use std::fs::File;
    use std::io::Write;
    use std::cmp::Ord;

    // Trait example
    fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(b"Aa");
        out.flush()
    }

    #[test]
    fn test_say_hello() {
        let mut local_file = File::create("hello.txt").expect("Unable to create file");
        let mut bytes: Vec<u8> = vec![];

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