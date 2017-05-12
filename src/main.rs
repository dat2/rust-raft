extern crate env_logger;
extern crate clap;

extern crate raft;

use clap::{Arg, App};

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

  let follower = raft::init_state();
  println!("{:?}", follower);
  let candidate = raft::election_timeout(follower);
  println!("{:?}", candidate);
  let follower = raft::other_leader_elected(candidate);
  println!("{:?}", follower);
  let follower = raft::vote(follower, String::from("nick"));
  println!("{:?}", follower);
  let follower = raft::complete_vote(follower);
  println!("{:?}", follower);

  Ok(())
}
