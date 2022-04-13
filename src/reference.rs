use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

static mut STASH: &i32 = &128;

fn show_table(table: Table) {
    for(kid, works) in table {
        println!("Work by {} :", kid);
        for work in works {
            println!("- {}", work);
        }
    }
}

fn show_table_by_reference(table: &Table) {
    for(kid, works) in table {
        println!("Work by {} :", kid);
        for work in works {
            println!("- {}", work);
        }
    }
}

fn sort_table(table: &mut Table) {
    for (_kid, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a , b | a * b)
}

fn smallest_in_vector(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

#[cfg(test)]
mod reference {
    use crate::reference::{factorial, show_table, show_table_by_reference, smallest_in_vector, sort_table, STASH, Table};

    # [test]
    fn problem_with_move_ownership() {
        let mut table = Table::new();
        table.insert("Duc Minh".to_string(),
                     vec!["studying".to_string(), "playing piano".to_string()]);
        table.insert("Ngoc Minh".to_string(),
                     vec!["writing".to_string(), "cleaning".to_string()]);

        show_table(table);

        // hashmap object has moved to function, so it can not be used
        // assert_eq!(table["Ngoc Minh"][0], "writing".to_string());
    }

    # [test]
    fn test_with_shared_ref() {
        let mut table = Table::new();
        table.insert("Duc Minh".to_string(),
                     vec!["studying".to_string(), "playing piano".to_string()]);
        table.insert("Ngoc Minh".to_string(),
                     vec!["writing".to_string(), "cleaning".to_string()]);

        show_table_by_reference(&table);

        // hashmap object has moved to function, so it can not be used
        assert_eq!(table["Ngoc Minh"][0], "writing".to_string());
    }

    #[test]
    fn test_with_mutable_ref() {
        let mut table = Table::new();
        table.insert("Duc Minh".to_string(),
                     vec!["studying".to_string(), "playing piano".to_string()]);
        table.insert("Ngoc Minh".to_string(),
                     vec!["writing".to_string(), "cleaning".to_string()]);

        sort_table(&mut table);

        assert_eq!(table["Ngoc Minh"][0], "cleaning".to_string());
    }

    #[test]
    fn test_dereference() {
        struct Profile {
            name: String,
            age: u8
        }

        struct Point {
            x: i32,
            y: i32
        }

        let my_profile = Profile {
            name: "Tuan Anh Le".to_string(),
            age: 40
        };

        let borrowed_profile = &my_profile;

        assert_eq!((*borrowed_profile).name, "Tuan Anh Le".to_string());
        assert_ne!((*borrowed_profile).age, 50);

        // reference to reference
        let point = Point { x: 10, y: 10 };
        let r: &Point = &point;
        let rr = &r;
        let rrr = &rr;

        assert_eq!(rrr.x, 10);
        assert_eq!(rrr.y, 10);

        // compare reference
        // dereference when compare
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert_eq!(x, y);
        assert_eq!(rx, ry);
        assert_eq!(rrx, rry);

        // checking reference compare by address of memory
        assert!(!std::ptr::eq(rx, ry));
        assert!(!std::ptr::eq(rrx, rry));

        // reference to expression
        let f = &factorial(6);
        assert_eq!(*f, 720);
        assert_eq!(f + &30, 750);
    }

    #[test]
    fn check_lifetime() {
        {
            // there are three life time
            // 1. r -> reference
            // 2. x -> variable
            // 3. &x -> reference borrow from x (variable)
            let r;
            {
                let x = 1;
                r = &x;
                assert_eq!(&x, &1);
            }

            // two rules
            // 1. variable lifetime must contain reference borrow from it
            // NOT WORKING
            // assert_eq!(&x, &1);

            // 2. reference lifetime must contain or enclose variable
            // NOT WORKING
            // assert_eq!(*r, 1);
            // error of borrowed value does not live long enough
            // (3) is not longer as (1)
        }

        {
            // variable lifetime contain reference borrow from it
            let x = 1;
            {
                // reference lifetime contain / enclose variable (y, or z)
                let y = &x;
                {
                    let z = &x;
                    assert_eq!(*z, 1);
                }
            }
        }
    }

    // lifetime of function `f` is just in its enclose
    fn f(p: &'static i32) {
        unsafe {
            // ERROR: lifetime of reference - STASH outlive the borrowed content p
            STASH = p;
        }
    }

    #[test]
    fn test_reference_in_parameters() {
        static p: i32 = 100;
        f(&p);
        unsafe {
            assert_eq!(STASH, &100);
        }
    }

    #[test]
    fn test_return_reference() {
        let s: &i32;
        {
            let p = [1, 3, 4, 5];
            let k = smallest_in_vector(&p);
            assert_eq!(*k, 1);
        }
    }

}