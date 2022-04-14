#[cfg(test)]
mod expression {
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
}