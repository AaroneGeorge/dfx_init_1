#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        // Test with a regular name
        let result = greet("Alice".to_string());
        assert_eq!(result, "Hello, Alice!");

        // Test with empty string
        let result = greet("".to_string());
        assert_eq!(result, "Hello, !");

        // Test with special characters
        let result = greet("@#$%".to_string());
        assert_eq!(result, "Hello, @#$%!");
    }
}
