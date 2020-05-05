# rofi-script

`rofi_script` is a wrapper for writing rust scripts for rofi, based on the specifications on [rofi-script](https://github.com/davatorium/rofi/blob/next/doc/rofi-script.5.markdown).

## Examples

See examples in [./examples](./examples) folder. Examples can be run with the following command:

```sh
cargo build --release --example <example_name>;
rofi -show fb -modi "fb:./target/release/examples/<example_name>"
```

## Usage

```rust
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
```
