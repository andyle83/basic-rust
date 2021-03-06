#[cfg(test)]
mod errorhandler {
    use std::string::ParseError;

    fn parseInt(str: String) -> Result<i32, String> {
        match str.parse::<i32>() {
            Ok(number) => {
                println!("parse successfully");
                Ok(number)
            },
            Err(e) => {
                println!("parse error");
                println!("Error in debug view {:?}", e);
                Err("Error".to_string())
            }
        }
    }

    #[test]
    fn test_parse() {
        let n1 = "10".to_string();
        assert_eq!(parseInt(n1).unwrap(), 10);

        let n2 = "20".to_string();
        assert!(parseInt(n2).is_ok());

        let b1 = "not_a_number".to_string();
        let b2 = b1.clone();
        let b3 = b2.clone();

        let result = match parseInt(b1) {
            Ok(a) => a + 10,
            Err(e) => {
                println!("Issue happen");
                0
            }
        };
        assert_eq!(result, 0);

        assert!(parseInt(b2).is_err());

        assert_eq!(parseInt(b3).unwrap_or(0), 0);

    }
}