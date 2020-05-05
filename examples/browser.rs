use rofi_script::{Retv, RofiContext, RofiMessage, RowOption};
use std::env;
use std::path::PathBuf;
use std::process;
use std::thread;

fn main() {
    let context = RofiContext::new();

    // Get the current directory
    let dir = match context.env.rofi_retv {
        Some(Retv::InitialCall) => env::current_dir().expect("Fail to get the current directory"),
        // In case rofi doesn't set ROFI_RETV e.g. in an earlier version, check the input
        None => match context.input.len() {
            0 => env::current_dir().expect("Fail to get the current directory"),
            _ => PathBuf::from(context.input),
        },
        _ => PathBuf::from(context.input),
    };

    let mut rows: Vec<RowOption> = Vec::new();

    if let Some(parent_dir) = dir.parent() {
        if let Some(parent_dir) = parent_dir.to_str() {
            rows.push(RowOption::new(parent_dir.to_string()));
        }
    }

    if !dir.is_dir() {
        process::Command::new("notify-send")
            .arg(dir)
            .spawn()
            .expect("Unable to open file");
        process::exit(0);
    };

    for entry in dir.read_dir().expect("Fail to read files in the directory") {
        if let Ok(entry) = entry {
            let mut row = match entry.path().into_os_string().into_string() {
                Ok(path) => RowOption::new(path),
                Err(_) => continue,
            };

            if let Ok(filetype) = entry.file_type() {
                if filetype.is_dir() {
                    row.set_icon("folder");
                }
                if filetype.is_file() {
                    row.set_icon("text-x-generic");
                }
            }
            rows.push(row);
        }
    }

    let rofi_message = RofiMessage {
        opt: Default::default(),
        row: rows,
    };

    print!("{}", rofi_message.to_string());
}
