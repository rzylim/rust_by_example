// cargo test test_foo

#[cfg(test)]
mod tests {
    #[test]
    fn test_bar() {}

    #[test]
    fn test_baz() {}

    #[test]
    fn test_foo_bar() {}

    #[test]
    fn test_foo() {}
}
