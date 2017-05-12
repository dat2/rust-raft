#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate rand;

mod errors;
mod state_machine;

pub use errors::Result;
pub use state_machine::*;
