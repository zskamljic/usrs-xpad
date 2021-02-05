#[derive(Debug)]
pub struct MetaKeys {
    pub select: bool,
    pub back: bool,
}
#[derive(Debug)]
pub struct AlphaKeys {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
}

#[derive(Debug)]
pub struct DirectionKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug)]
pub struct BackKeys {
    pub lb: bool,
    pub rb: bool,
    pub lt: u16,
    pub rt: u16,
}

#[derive(Debug)]
pub struct Stick {
    pub x: i16,
    pub y: i16,
    pub clicked: bool,
}

#[derive(Debug)]
pub enum Command {
    Terminate,
    Keys {
        meta: MetaKeys,
        alpha: AlphaKeys,
        direction: DirectionKeys,
        back: BackKeys,
        left_stick: Stick,
        right_stick: Stick,
    },
    Xbox(bool),
}
