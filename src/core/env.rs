/// Environment variable `ROFI_RETV` showing the current state
pub enum Retv {
    InitialCall,
    EntrySelected,
    CustomEntrySelected,
    CustomKey(i8),
}

impl From<i8> for Retv {
    fn from(value: i8) -> Self {
        match value {
            0 => Retv::InitialCall,
            1 => Retv::EntrySelected,
            2 => Retv::CustomEntrySelected,
            10..=28 => Retv::CustomKey(value - 9),
            _ => unreachable!(),
        }
    }
}
