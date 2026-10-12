use std::io;

pub fn get_user_expression() -> Option<String> {
    let mut user_expression: String = String::new();
    io::stdin().read_line(&mut user_expression).unwrap();
    let trimmed = user_expression.trim();
    if trimmed.eq_ignore_ascii_case("exit") {
        return None;
    }
    Some(trimmed.to_string())
}