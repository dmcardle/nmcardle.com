const GAME: &str = "Foo";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!("foo", "foo");
        assert_eq!(str, "Foo");
    }
}
