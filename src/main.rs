use evalexpr::*;
use std::io;

fn get_user_expression() -> Option<String> {
    let mut user_expression: String = String::new();
    io::stdin().read_line(&mut user_expression).unwrap();
    let trimmed = user_expression.trim();
    if trimmed.eq_ignore_ascii_case("exit") {
        return None;
    }
    Some(trimmed.to_string())
}

fn main() {
    loop {
        let expr = match get_user_expression() {
            Some(e) => e,
            None => {
                println!("Good Bye!");
                break;
            }
        };

        match eval(&expr) {
            Ok(v) => println!("={}", v),
            Err(e) => println!("Error:{}", e),
        }
    }
}
