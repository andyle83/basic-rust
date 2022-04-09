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
            team.push(p1);
            team.push(p2);

            // team has been move into this
            for member in team {
                println!("{} with age {}", member.name, member.age);
            }

            // so now we can NOT borrow
            assert_eq!(team.len(), 2);
        }

        // tuple, array, vector own contained element
    }
}