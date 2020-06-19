// https://github.com/serde-rs/serde/blob/b87f8f35ee631ff5fb8f01e0ebf5ad1f5148d369/serde_derive/src/internals/case.rs#L46
// snakecase

pub fn snake_case(variant: &String) -> String {
    let mut snake = String::new();
    for (i, ch) in variant.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}
