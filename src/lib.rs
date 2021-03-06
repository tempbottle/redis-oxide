// #![deny(unsafe_code)]
#![feature(test, async_closure)]
#![warn(clippy::all, clippy::nursery)]
#![feature(const_fn)]
extern crate test;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate slog;

extern crate rmp_serde as rmps;

pub mod asyncresp;
pub mod blocking;
pub mod bloom;
pub mod database;
pub mod hashes;
pub mod keys;
pub mod lists;
pub mod logger;
#[macro_use]
pub mod macros;
pub mod data_structures;
pub mod misc;
pub mod ops;
pub mod server;
pub mod sets;
pub mod sorted_sets;
pub mod stack;
pub mod startup;
pub mod state;
pub mod timeouts;
pub mod types;
