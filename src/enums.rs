#[cfg(test)]
mod enums {
    use std::cmp::Ordering;
    use http::StatusCode;
    use std::mem::size_of;

    fn http_status_from_u32(n: u32) -> Option<StatusCode> {
        match n {
            200 => Some(StatusCode::OK),
            304 => Some(StatusCode::NOT_MODIFIED),
            404 => Some(StatusCode::NOT_FOUND),
            _ => None
        }
    }

    #[test]
    fn test_builtin_enum() {
        assert_eq!(size_of::<Ordering>(), 1);
        assert_eq!(Ordering::Less as i32, -1);
        assert_eq!(Ordering::Equal as i32, 0);
        assert_eq!(Ordering::Greater as i32, 1);

        assert_eq!(size_of::<StatusCode>(), 2);
        assert_eq!(StatusCode::OK, 200);
        assert_eq!(StatusCode::NOT_FOUND, 404);
    }

    #[test]
    fn test_status_code_converter() {
        assert_eq!(http_status_from_u32(200).unwrap(), StatusCode::OK);
        assert_eq!(http_status_from_u32(100), None);
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum TimeUnit {
        Seconds, Minutes, Hours, Days, Months, Years
    }

    impl TimeUnit {
        fn plural(self) -> &'static str {
            match self {
                TimeUnit::Seconds => "seconds",
                TimeUnit::Minutes => "minutes",
                TimeUnit::Hours => "hours",
                TimeUnit::Days => "days",
                TimeUnit::Months => "months",
                TimeUnit::Years => "years"
            }
        }

        fn singular(self) -> &'static str {
            self.plural().trim_matches('s')
        }
    }

    #[test]
    fn test_time_unit() {
        let hours = TimeUnit::Hours;

        assert_eq!(hours.plural(), "hours");
        assert_eq!(hours.singular(), "hour");
    }

    // Example data contain enum
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum RoughTime {
        InThePass(TimeUnit, u32),
        JustNow,
        InTheFuture(TimeUnit, u32)
    }

    impl RoughTime {
        fn print_rough_time(self) -> String {
            match self {
                RoughTime::JustNow => "Just in time".to_string(),
                RoughTime::InThePass(unit, length) => format!("Last {:?} {:?}", length, unit.plural()),
                RoughTime::InTheFuture(unit, length) => format!("Until {:?} {:?}", length, unit.plural())
            }
        }
    }

    #[test]
    fn test_rough_time() {
        let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
        println!("{}", three_hours_from_now.print_rough_time());
    }


    // Struct variant inside enum
    struct Point3D {
        x: u32,
        y: u32,
        z: u32
    }

    enum Shape {
        Sphere {
            center: Point3D,
            radius: u32
        },
        Cuboid {
            corner1: Point3D,
            corner2: Point3D
        }
    }

    impl Shape {
        fn get_radius(self) -> Option<u32> {
            match self {
                Shape::Sphere { center, radius} => Some(radius),
                _ => None,
            }
        }
    }

    #[test]
    fn enum_contain_struct_variant() {
        let shape = Shape::Sphere {
            center: Point3D {
                x: 0,
                y: 0,
                z: 0
            },
            radius: 100
        };

        // How to unpack Shape to get radius value
        let radius = shape.get_radius().unwrap_or(0);

        assert_eq!(radius, 100);
    }

    fn match_slice_data(names: &[&str]) {
        match names {
            [] => println!("Hello, Mr. Nobody"),
            [a] => println!("Hello, Mr (Mrs) {}", a),
            [a, b] => println!("Hello, {} and {}", a, b),
            [a, .., b] => println!("Hello, {}....{}", a, b)
        }
    }

    #[test]
    fn test_match_slice_data() {
        let single = ["Anh Le"];
        let plural = ["Duc Minh", "Ngoc Minh", "Mai Huong"];
        match_slice_data(&single);
        match_slice_data(&plural);
    }

    // Using a complicate data struct
    enum Status {
        Single,
        Married
    }

    struct  Account<'a> {
        name: &'a str,
        language: &'a str,
        id: u32,
        status: Status,
        address: String
    }

    #[test]
    fn test_match_complicated_structure() {
        let account =  Account {
            name: "Tuan Anh Le",
            language: "Viet Nam",
            id: 100,
            status: Status::Single,
            address: "4/12 Mincha Street".to_string()
        };

        match account {
            Account {name, language, ..} => {
                greeting(name, language);
                show_settings(&account)
            },
            _ => println!("Not found any account")
        }
    }

    fn greeting(name: &str, language: &str) {
        println!("Hello {} by language {} ", name, language);
    }

    fn show_settings(account: &Account) {
        println!("Name {}", account.name);
        println!("Lang {}", account.language);
    }
}