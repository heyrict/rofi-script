pub mod core;

pub use crate::core::{
    env::Retv,
    option::{ModeOption, RowOption},
};

/// All environment variables provided by rofi
pub struct RofiEnv {
    pub rofi_retv: Option<Retv>,
}

impl RofiEnv {
    /// Create a new RofiEnv from current environment variables
    fn new() -> Self {
        let rofi_retv = std::env::var("ROFI_RETV")
            .ok()
            .and_then(|var| var.parse::<i8>().ok().map(Retv::from));

        RofiEnv { rofi_retv }
    }
}

/// Context provided by rofi
pub struct RofiContext {
    pub env: RofiEnv,
    pub input: String,
}

impl RofiContext {
    /// Load rofi context
    pub fn new() -> Self {
        let env = RofiEnv::new();

        let args = std::env::args();
        let args: Vec<_> = args.into_iter().skip(1).collect();
        let input = args.join(" ");

        RofiContext { env, input }
    }
}

/// Message that will be sent to rofi
pub struct RofiMessage<'a> {
    pub opt: ModeOption<'a>,
    pub row: Vec<RowOption<'a>>,
}

impl<'a> RofiMessage<'a> {
    pub fn to_string(&self) -> String {
        let mut output = String::new();

        // Add options to the beginning
        if !self.opt.is_empty() {
            output.push_str(&self.opt.to_string());
        }

        // Add rows
        for row_option in self.row.iter() {
            output.push_str(&row_option.to_string());
        }

        output
    }
}
