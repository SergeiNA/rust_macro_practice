use proc_macro_try_from::FromPrimitive;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(FromPrimitive, Debug, PartialEq, Eq)]
    enum Color {
        Red,
        Green,
        Blue,
        Yellow,
    }

    #[test]
    fn test_from_primitive() {
        assert_eq!(Color::Red as usize, 0);
        assert_eq!(Color::Green as usize, 1);
        assert_eq!(Color::Blue as usize, 2);
        assert_eq!(Color::Yellow as usize, 3);

        for (i, val) in
            (0..3).zip([Color::Red, Color::Green, Color::Blue, Color::Yellow].into_iter())
        {
            assert_eq!(Color::try_from(i), Ok(val));
        }
        assert!(Color::try_from(100).is_err());
    }
}
