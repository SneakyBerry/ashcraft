pub mod common;
pub mod object;
pub mod player;
pub mod unit;

pub mod prelude {
    pub use bevy_transform::*;
    pub use bevy_math::prelude::*;
}

#[macro_use]
extern crate derive_builder;
