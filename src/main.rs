extern crate env_logger;
extern crate clap;

extern crate raft;

use clap::{Arg, App};
use raft::Server;

fn main() {
  if let Err(ref e) = run() {
    println!("error: {}", e);

    for e in e.iter().skip(1) {
      println!("caused by: {}", e);
    }

    if let Some(backtrace) = e.backtrace() {
      println!("backtrace: {:?}", backtrace);
    }

    ::std::process::exit(1);
  }
}

fn run() -> raft::Result<()> {
  env_logger::init()?;

  let addr = "127.0.0.1:12345".parse()?;

  let mut server = Server::new()?;
  server.start(&addr)?;

  Ok(())
}
