#![forbid(unsafe_code)]
#![allow(non_shorthand_field_patterns)]

extern crate secp256k1;

extern crate wire;
extern crate common_types;
extern crate hmac;
extern crate chacha;
extern crate sha2;
extern crate serde;
extern crate serde_derive;

#[cfg(test)]
extern crate rand;

mod crypto;
mod hop;
mod packet;
mod route;

#[cfg(test)]
mod tests;

pub use self::route::{OnionPacketVersion, OnionRoute};
pub use self::packet::{OnionPacket, ValidOnionPacket, Processed, OnionPacketProcessingError};
pub use self::hop::{Hop, HopData, BitcoinHopData};