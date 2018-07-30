#![forbid(unsafe_code)]
#![allow(non_shorthand_field_patterns)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate secp256k1;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
extern crate rand;

mod message;

mod serde_facade;

pub use self::message::Message;
pub use self::message::types::*;
pub use self::message::channel::*;
pub use self::message::setup::*;
pub use self::message::control::*;

pub use self::serde_facade::BinarySD;
