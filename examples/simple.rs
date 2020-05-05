use rofi_script::{RofiContext, RofiMessage, RowOption};
use std::process;

fn main() {
    let context = RofiContext::new();

    let rofi_message = match context.input.as_str() {
        "quit" => process::exit(0),
        _ => {
            let row = vec![RowOption::new("reload"), RowOption::new("quit")];
            RofiMessage {
                opt: Default::default(),
                row,
            }
        }
    };

    print!("{}", rofi_message.to_string());
}
