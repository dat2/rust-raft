use std::io;
use std::error::Error;
use std::net::SocketAddr;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use bincode::{serialize, deserialize, Infinite};
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;

use errors;
use messages::{RaftRequest, RaftResponse};

struct RaftCodec;

impl Decoder for RaftCodec {
  type Item = RaftRequest;
  type Error = io::Error;

  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<RaftRequest>> {
    if let Some(i) = buf.iter().position(|&b| b == b'\n') {
      let line = buf.split_to(i);
      buf.split_to(1);
      match deserialize(&line) {
        Ok(msg) => Ok(Some(msg)),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.description()))
      }
    } else {
      Ok(None)
    }
  }
}

impl Encoder for RaftCodec {
  type Item = RaftResponse;
  type Error = io::Error;

  fn encode(&mut self, msg: RaftResponse, buf: &mut BytesMut) -> io::Result<()> {
    match serialize(&msg, Infinite) {
      Ok(bytes) => {
        buf.extend(bytes);
        buf.extend(b"\n");
        Ok(())
      },
      Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.description()))
    }
  }
}

struct RaftProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for RaftProto {
  type Request = RaftRequest;
  type Response = RaftResponse;
  type Transport = Framed<T, RaftCodec>;
  type BindTransport = io::Result<Self::Transport>;

  fn bind_transport(&self, io: T) -> Self::BindTransport {
    Ok(io.framed(RaftCodec))
  }
}

struct Raft;

impl Service for Raft {
  type Request = RaftRequest;
  type Response = RaftResponse;
  type Error = io::Error;
  type Future = BoxFuture<Self::Response, Self::Error>;

  fn call(&self, req: Self::Request) -> Self::Future {
    info!(target: "rust", "[SERVER] request {:?}", req);
    match req {
      RaftRequest::RequestVote(_term, _candidate) => future::ok(RaftResponse::Vote(_term, true)).boxed(),
      RaftRequest::Heartbeat => future::ok(RaftResponse::Heartbeat).boxed()
    }
  }
}

pub fn serve(addr: SocketAddr) -> errors::Result<()> {
  let addr_display = addr.to_string();
  let server = TcpServer::new(RaftProto, addr);
  info!(target: "raft", "[SERVER] starting on {:?}", addr_display);
  server.serve(|| Ok(Raft));
  Ok(())
}
