#[cfg(test)]
mod types {
    #[test]
    fn byte_literal_check() {
        // byte literal - ASC characters only
        // emphasize value represent ASC character
        let a = b'a';
        let capital_a = b'A';

        println!("{:?}", a);
        println!("{:?}", capital_a);

        assert_eq!(a, 97);
        assert_eq!(capital_a, 65);
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

        // method in character
        assert_eq!('a'.is_alphabetic(), true);
        assert_eq!('8'.to_digit(10), Some(8));
        assert_eq!(std::char::from_digit(8, 10), Some('8'));
    }

    #[test]
    fn tuple_type_check() {
        // Type: group of different value with different types
        let text = "12345";
        let (head, tail) = text.split_at(2);

        println!("head {:?}", head);
        println!("tail {:?}", tail);

        assert_eq!(head, "12");
        assert_eq!(tail, "345");
    }

    #[test]
    fn pointer_type_check() {
        // pointer represent address of a memory (heap, stack)
        // types of pointer
        // 1. reference
        // 2. boxes
        // 3. unsafe pointer

        // 1. reference
        // Giving a x value
        // &x produce a reference to x - borrow reference to x
        // NO way to create a NULL reference
        // 1. &x - immutable reference or read only
        // 2. &mut x - mutable reference with write permit
        {
            let x = 10;
            let r = &x;
            let p = &x;
            assert_eq!(r, p);
            assert_eq!(*r, 10);
        }

        // 2. boxes
        // Giving a way to allocate value in head
        {
            let profile = (40, "Tuan Anh");
            let profile_box = Box::new(profile);
            let (age, name) = *profile_box;
            assert_eq!(age, 40);
            assert_eq!(name, "Tuan Anh");
        }

        // 3. raw point
        // no tracking
        // may be null
        // may point to free memory
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

        // there are few ways to create string
        let str1 = "this is the first way".to_string();
        let str2 = format!("{} {} {} {}", "this", "is", "second", "way");
        let str3 = vec!["this", "is", "third", "way"].concat();
        let str4 = vec!["this", "is", "fourth", "way"].join( " ");

        assert_eq!(str1.to_uppercase(), "THIS IS THE FIRST WAY");
        assert!(str2.contains("this"));
        assert!(str3.starts_with("this"));
        assert!(str4.ends_with("way"));

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
}
