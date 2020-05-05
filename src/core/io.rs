use super::option::{ModeOption, RowOption};
use std::fmt;

enum RofiOutput<'a> {
    Opt(ModeOption<'a>),
    Row(RowOption<'a>),
}

impl<'a> From<ModeOption<'a>> for RofiOutput<'a> {
    fn from(mode_option: ModeOption<'a>) -> Self {
        RofiOutput::Opt(mode_option)
    }
}

impl<'a> From<RowOption<'a>> for RofiOutput<'a> {
    fn from(row_option: RowOption<'a>) -> Self {
        RofiOutput::Row(row_option)
    }
}

impl<'a> fmt::Display for RofiOutput<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'a> RofiOutput<'a> {
    fn to_string(&self) -> String {
        match self {
            RofiOutput::Opt(mode_option) => mode_option.to_string(),
            RofiOutput::Row(row_option) => row_option.to_string(),
        }
    }
}
