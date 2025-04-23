pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        // Arrange
        let a = 2;
        let b = 3;
        let expected = 5;
        // Act
        let result = add_two(a, b);
        // Assert
        assert_eq!(
            result, expected,
            "Expected {} + {} to equal {}",
            a, b, expected
        );
    }
}
