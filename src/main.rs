fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
        assert_eq!(1, 1); // Dummy assertion to ensure the test runs
    }
}
