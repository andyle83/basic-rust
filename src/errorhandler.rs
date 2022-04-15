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
                Err("Error".to_string())
            }
        }
    }

    #[test]
    fn test_parse() {
        let a = "10".to_string();
        assert_eq!(parseInt(a).unwrap(), 10);

        let b = "not_a_nunber".to_string();
        let result = match parseInt(b) {
            Ok(a) => a + 10,
            Err(e) => {
                println!("Issue happen");
                0
            }
        };

        assert_eq!(result, 0);

    }
}