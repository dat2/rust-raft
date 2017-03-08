#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

extern crate futures;
extern crate tokio_core;

extern crate serde;

mod errors;
mod raft_log;
mod server;

pub use server::Server;
pub use errors::Result;
