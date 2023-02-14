use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub enum Expansion {
    Vanilla = 0x0,
    TheBurningCrusade = 0x1,
    WrathOfTheLichLing = 0x2,
}
