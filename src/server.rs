use std::net::SocketAddr;

use futures::{Future, Stream};
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

use log::*;
use errors::*;

pub struct Server {
  core: Core,
}

impl Server {
  pub fn new() -> Result<Server> {
    let core = Core::new()?;
    Ok(Server { core: core })
  }

  pub fn start(&mut self, addr: &SocketAddr) -> Result<()> {
    info!(target: "raft", "the server is starting on address: {}", addr);

    let handle = self.core.handle();
    let sock = TcpListener::bind(addr, &handle)?;

    // Pull out a stream of sockets for incoming connections
    let server = sock.incoming().for_each(|(socket, _)| {
      // Split up the reading and writing parts of the
      // socket
      let (reader, writer) = socket.split();

      // A future that echos the data and returns how
      // many bytes were copied...
      let bytes_copied = copy(reader, writer);

      // ... after which we'll print what happened
      let handle_conn = bytes_copied.map(|amt| println!("wrote {} bytes", amt))
        .map_err(|err| println!("IO error {:?}", err));

      // Spawn the future as a concurrent task
      handle.spawn(handle_conn);

      Ok(())
    });

    // Spin up the server on the event loop
    self.core.run(server)?;

    Ok(())
  }
}
