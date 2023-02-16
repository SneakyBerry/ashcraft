pub mod common;
pub mod object;
pub mod player;
pub mod unit;
pub mod objects;

pub mod prelude {
    pub use bevy_transform::*;
}

#[macro_use]
extern crate derive_builder;

pub(crate) mod crate_macro {
    #[macro_export]
    macro_rules! impl_update {
        ($self:ident, $old:ident, $update:ident { $($source:ident => $target:ident),* }) => {
            $ (
                $update.$target = if let Some($source) = $old.map(|o| &o.$source) {
                    if $source == &$self.$source {
                            Some($self.$source.clone())
                    } else {
                            None
                    }
                } else {
                    None
                };
            ) *
        };
    }
}
