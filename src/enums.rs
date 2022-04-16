#[cfg(test)]
mod enums {
    use std::cmp::Ordering;
    use http::StatusCode;
    use std::mem::size_of;

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
}