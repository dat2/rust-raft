extern crate env_logger;
extern crate clap;
extern crate tokio_core;
extern crate futures;
extern crate raft;

use std::error::Error;
use std::net::{SocketAddr,AddrParseError};
use std::thread;
use std::time::Duration;
use clap::{Arg, App};
use tokio_core::reactor::Core;

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

fn validate_socket_addr(v: String) -> Result<(), String> {
  let parse_result: Result<SocketAddr, AddrParseError> = v.parse();
  match parse_result {
    Ok(_) => Ok(()),
    Err(e) => Err(e.description().to_string())
  }
}

fn run() -> raft::Result<()> {
  env_logger::init()?;

  let matches = App::new("raftd")
    .version("0.1")
    .author("Nicholas D. <nickdujay@gmail.com>")
    .about("Runs a simple raft server.")
    .arg(Arg::with_name("bind")
      .short("b")
      .long("bind")
      .value_name("BIND")
      .help("Sets the bind address")
      .takes_value(true)
      .validator(validate_socket_addr)
      .required(true))
    .arg(Arg::with_name("connect")
      .short("c")
      .long("connect")
      .help("Sets the addresses of the bootup cluster")
      .takes_value(true)
      .validator(validate_socket_addr)
      .multiple(true)
      .use_delimiter(false)
      .required(true))
    .get_matches();

  let bind = matches.value_of("bind").unwrap();
  let connect: Vec<_> = matches.values_of("connect").unwrap().collect();

  let bind_addr: SocketAddr = bind.parse()?;
  thread::spawn(move|| {
    raft::serve(bind_addr.clone()).unwrap();
  });

  // wait 5 seconds
  thread::sleep(Duration::from_secs(2));

  let mut lp = Core::new()?;
  let lp_handle = lp.handle();

  for client_addr in &connect {
    let addr = client_addr.parse()?;
    raft::run_client(addr, &lp_handle)?;
  }

  loop {
    lp.turn(None)
  }
}
