use proc_macro_count::Count;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Count, Debug, PartialEq, Eq)]
    enum Color {
        Red,
        Green,
        Blue,
        Yellow,
    }

    #[derive(Count, Debug, PartialEq, Eq)]
    enum Empty {}

    #[test]
    fn test_count() {
        assert_eq!(Color::count(), 4);
    }

    #[test]
    fn test_empty_enum() {
        // Test that an empty enum still counts its variants
        assert_eq!(Empty::count(), 0);
    }
}
