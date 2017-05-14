#[derive(Serialize, Deserialize, PartialEq,Debug)]
pub enum RaftRequest {
  RequestVote(usize, String),
  Heartbeat
}

#[derive(Serialize, Deserialize, PartialEq,Debug)]
pub enum RaftResponse {
  Vote(usize, bool),
  Heartbeat
}
