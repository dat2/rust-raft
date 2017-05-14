use std::io;
use std::error::Error;
use std::net::SocketAddr;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use bincode::{serialize, deserialize, Infinite};
use tokio_proto::pipeline::ClientProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::Future;
use tokio_core::reactor::Handle;
use tokio_proto::TcpClient;

use errors;
use messages::{RaftRequest, RaftResponse};

pub struct RaftClientCodec;

impl Decoder for RaftClientCodec {
  type Item = RaftResponse;
  type Error = io::Error;

  // bytesmut -> simple & efficient buffer management
  // try to extract the first complete message, if there is one
  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<RaftResponse>> {
    if let Some(i) = buf.iter().position(|&b| b == b'\n') {
      let line = buf.split_to(i);
      buf.split_to(1);
      match deserialize(&line) {
        Ok(msg) => Ok(Some(msg)),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.description())),
      }
    } else {
      Ok(None)
    }
  }
}

impl Encoder for RaftClientCodec {
  type Item = RaftRequest;
  type Error = io::Error;

  fn encode(&mut self, msg: RaftRequest, buf: &mut BytesMut) -> io::Result<()> {
    match serialize(&msg, Infinite) {
      Ok(bytes) => {
        buf.extend(bytes);
        buf.extend(b"\n");
        Ok(())
      }
      Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.description())),
    }
  }
}

pub struct RaftClientProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for RaftClientProto {
  type Request = RaftRequest;
  type Response = RaftResponse;
  type Transport = Framed<T, RaftClientCodec>;
  type BindTransport = io::Result<Self::Transport>;

  fn bind_transport(&self, io: T) -> Self::BindTransport {
    Ok(io.framed(RaftClientCodec))
  }
}

pub fn run_client(addr: SocketAddr, handle: &Handle) -> errors::Result<()> {
  let id = addr.to_string();
  let client = TcpClient::new(RaftClientProto);
  let future = client.connect(&addr, handle)
    .and_then(|service| service.call(RaftRequest::RequestVote(1, id)))
    .map(|res| {
      println!("[CLIENT] {:?}", res);
      ()
    })
    .map_err(|_| ());
  handle.spawn(future);

  Ok(())
}
