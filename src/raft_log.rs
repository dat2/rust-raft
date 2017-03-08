
use std::clone::Clone;
use std::sync::{Mutex, RwLock};
use serde::Serialize;

#[derive(Debug)]
struct RaftLogMessage<T: Serialize> {
  id: usize,
  data: T,
}

#[derive(Debug)]
pub struct RaftLog<T: Serialize> {
  messages: RwLock<Vec<RaftLogMessage<T>>>,
  id: Mutex<usize>,
}

impl<T: Serialize + Clone> RaftLog<T> {
  pub fn new() -> RaftLog<T> {
    RaftLog {
      messages: RwLock::new(Vec::new()),
      id: Mutex::new(0),
    }
  }

  fn get_id(&self) -> usize {
    // TODO make this non blocking :)
    let mut guard = self.id.lock().unwrap();
    let value = *guard;
    *guard += 1;
    value
  }

  pub fn append(&mut self, data: T) {
    let id = self.get_id();
    let mut list = self.messages.write().unwrap();
    list.push(RaftLogMessage {
      id: id,
      data: data,
    });
  }

  pub fn get(&self, index: usize) -> Option<T> {
    let list = self.messages.read().unwrap();
    let data = list[index].data.clone();

    if index < list.len() { Some(data) } else { None }
  }
}

// consensus
// leader election - randomized timers
// strong leader - Raftlog entries flow from leader to others
// Raftlog replication
//
// safety
// heartbeats

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_append() {
    let mut Raftlog = RaftLog::new();
    Raftlog.append(String::from("hello world"));
    assert_eq!(Raftlog.get(0), Some(String::from("hello world")));
  }

}
