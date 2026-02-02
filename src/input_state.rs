/// Input state for emulator controls
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub start: bool,
    pub select: bool,
    pub l: bool,
    pub r: bool,
}
