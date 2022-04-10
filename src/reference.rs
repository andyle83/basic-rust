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

#[cfg(test)]
mod reference {
    use crate::reference::{show_table, show_table_by_reference, Table};

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
    fn solve_problem_with_shared_reference() {
        let mut table = Table::new();
        table.insert("Duc Minh".to_string(),
                     vec!["studying".to_string(), "playing piano".to_string()]);
        table.insert("Ngoc Minh".to_string(),
                     vec!["writing".to_string(), "cleaning".to_string()]);

        show_table_by_reference(&table);

        // hashmap object has moved to function, so it can not be used
        assert_eq!(table["Ngoc Minh"][0], "writing".to_string());
    }

}