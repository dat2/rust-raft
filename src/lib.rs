#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate rand;

extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

extern crate bincode;
#[macro_use]
extern crate serde_derive;

mod errors;
mod server;
mod client;
mod messages;
mod state;

pub use errors::Result;
pub use server::*;
pub use client::*;
