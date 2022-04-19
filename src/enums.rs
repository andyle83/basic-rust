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
}