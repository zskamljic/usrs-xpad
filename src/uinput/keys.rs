use std::os::raw::c_char;

pub enum Key {
    A,
    B,
    X,
    Y,
    LB,
    RB,
    Back,
    Select,
    Xbox,
}

impl Key {
    pub fn map(self) -> c_char {
        (match self {
            Key::A => 'S',
            Key::B => 'E',
            Key::X => 'N',
            Key::Y => 'D',
            Key::LB => 'U',
            _ => ' ',
        }) as c_char
    }
}
