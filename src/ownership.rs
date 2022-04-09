#[cfg(test)]
mod ownership {
    #[test]
    fn box_ownership() {
        {
            let point = Box::new((1, 2));
            let label = format!("{:?}", point);
            assert_eq!(label, "(1, 2)");
        }
    }

    #[test]
    fn struct_ownership() {
        // struct onw their field
        struct Person {
            name: String,
            age: i32
        }

        {
            let p1 = Person {
                name: "Anh Le".to_string(),
                age: 40
            };

            let p2 = Person {
                name: "Ngoc Minh".to_string(),
                age: 10
            };

            let mut team = Vec::new();

            // mover ownership p1 and p2 to team
            // NOTE:
            // - Every value has a single owner
            // - A single value may own many other values.
            team.push(p1);
            team.push(p2);

            let name = std::mem::replace(&mut team[1].name, "Duc Minh".to_string());
            assert_eq!(name, "Ngoc Minh".to_string());

            // team has been move into this
            for member in &team {
                println!("{} with age {}", member.name, member.age);
            }

            // so now we can NOT borrow
            assert_eq!(team.len(), 2);
        }

        {
            // tuple, array, vector own contained element}
            let v = vec!["le".to_string(), "tuan".to_string(), "anh".to_string()];

            // move ownership of vector to t, v is uninitialized
            let t = v;

            // this is not working because v is uninitialized
            // let u = v;

            // and now t and v are uninitialized
            let u = t;
            assert_eq!(u.len(), 3);
        }

        {
            // perform clone to make deep copy, and therefore avoid moving ownership
            let v = vec!["le".to_string(), "tuan".to_string(), "anh".to_string()];
            let mut t = v.clone();
            let mut u = t.clone();
            assert_eq!(t.len(), 3);
            assert_eq!(u.pop().unwrap(), "anh".to_string());
        }
    }

    #[derive(Copy, Clone)]
    struct Point {
        x: u32,
        y: u32
    }

    fn print_point(p: Point) {
        println!("Point info: {} {}", p.x, p.y);
    }

    #[test]
    fn copy() {
        let p = Point {
            x: 100,
            y: 200
        };

        print_point(p);

        println!("Point format: ({},{})", p.x, p.y);
    }
}