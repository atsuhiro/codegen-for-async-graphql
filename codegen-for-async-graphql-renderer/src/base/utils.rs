// https://github.com/serde-rs/serde/blob/b87f8f35ee631ff5fb8f01e0ebf5ad1f5148d369/serde_derive/src/internals/case.rs#L46
// snakecase

#[must_use]
pub fn snake_case(variant: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in variant.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_to_snake_case() {
        let snake = "very_tasty".to_string();
        ["VeryTasty", "veryTasty"].iter().for_each(|f| {
            assert_eq!(snake, snake_case(f));
        });
    }
}
