use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

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

#[cfg(test)]
mod reference {
    use crate::reference::{show_table, show_table_by_reference, sort_table, Table};

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
    }

}