use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct LFGLeave;

    #[deku_derive(DekuRead)]
    #[derive(Debug, Clone)]
    pub struct LFGJoin {
        pub roles: u32,
        pub no_partial_clear: bool,
        pub achievements: bool,
        #[deku(temp)]
        slots_size: u8,
        #[deku(count = "slots_size")]
        pub slots: Vec<u32>,
        #[deku(temp)]
        needs_size: u8,
        #[deku(count = "needs_size")]
        pub needs: Vec<u8>,
    }
}
