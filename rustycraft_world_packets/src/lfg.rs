use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct LFGLeave;

    #[derive(Debug, Clone, DekuRead)]
    pub struct LFGJoin {
        pub roles: u32,
        pub no_partial_clear: bool,
        pub achievements: bool,
        pub slots_size: u8,
        #[deku(count = "slots_size")]
        pub slots: Vec<u32>,
        pub needs_size: u8,
        #[deku(count = "needs_size")]
        pub needs: Vec<u8>,
    }
}
