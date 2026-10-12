use evalexpr::*;
mod get_expression;
use get_expression::get_user_expression;

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
