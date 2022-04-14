#[cfg(test)]
mod expression {

    // Diverging function
    fn foo() -> ! {
        panic!("This call never return");
    }

    fn bar() {
        ()
    }

    #[test]
    fn test_divergent() {
        let a = bar();
        assert_eq!(a, ());
        foo();
        // Never call here
        assert_eq!(1, 0);
    }

    #[test]
    fn check_if_else() {
        let b = true;
        let m = if b {
            println!("True");
            1
        } else {
            println!("False");
            0
        };

        assert_eq!(m, 1);

        if let Some(a) = Some(10) {
            assert_eq!(a, 10);
        } else {
            println!("Error");
        }
    }

    #[test]
    fn check_match_expression() {
        let code;

        code = 100;

        let _result = match code {
            0 => println!("0"),
            1 => println!("1"),
            _ => println!("Neither 0 or 1")
        };

        let result = Some(100);

        match result {
            Some(a) => println!("{}", a),
            _ => println!("Nothing")
        };

        assert_eq!(_result, ());
    }

    #[test]
    fn check_loop() {
        let results = vec!["hello", "goodbye"];

        for result in &results {
            println!("{}", result);
        }

        assert_eq!(results.len(), 2);
    }
}