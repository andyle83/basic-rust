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
    }
}