#[macro_export]
macro_rules! assert_type {
    ($x:expr, $t:ty) => {
        {
            let _: $t = $x;  // Verify type at compile time by assigning the expression to a variable of the expected type. 
            // This ensures that the expression's type matches the expected type, leveraging Rust's type-checking system.
        }
    };
}

// Example usage in tests
#[cfg(test)]
mod tests {

    #[test]
    fn test_type_checking() {
        let x = 42;
        assert_type!(x, i32);  // Passes
        
        let s = String::from("hello");
        assert_type!(s, String);  // Passes
        
        // This would fail at compile time:
        // assert_type!(x, String);
    }
}