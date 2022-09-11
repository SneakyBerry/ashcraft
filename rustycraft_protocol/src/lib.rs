pub mod autogen_impls;
pub mod expansions;
pub mod messages;
pub mod rpc_responses;

mod autogen {
    include!(concat!(env!("OUT_DIR"), "/autogen.rs"));
}

pub use autogen::bgs;
pub use autogen::blizzard;
pub use autogen::google;
