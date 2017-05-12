#[derive(Debug,Clone)]
pub struct Raft<S> {
  pub state: S,
  pub term: usize
}

impl<S> Raft<S> {
  fn new(state: S, term: usize) -> Raft<S> {
    Raft {
      state: state,
      term: term,
    }
  }
}

#[derive(Debug,Clone)]
pub struct Follower {
  voted_for: Option<String>
}

#[derive(Debug,Clone)]
pub struct Candidate;

#[derive(Debug,Clone)]
pub struct Leader;

impl Follower {
  fn new(voted_for: Option<String>) -> Follower {
    Follower { voted_for: voted_for }
  }
}

// follower to candidate
impl From<Raft<Follower>> for Raft<Candidate> {
  fn from(val: Raft<Follower>) -> Raft<Candidate> {
    Raft::new(Candidate, val.term + 1)
  }
}

// candidate to follower (eg. another leader has been elected)
impl From<Raft<Candidate>> for Raft<Follower> {
  fn from(val: Raft<Candidate>) -> Raft<Follower> {
    Raft::new(Follower::new(None), val.term + 1)
  }
}

// candidate to leader (eg. candidate successfully hit a majority)
impl From<Raft<Candidate>> for Raft<Leader> {
  fn from(val: Raft<Candidate>) -> Raft<Leader> {
    Raft::new(Leader, val.term + 1)
  }
}

// everyone starts off as a follower
pub fn init_state() -> Raft<Follower> {
  Raft::new(Follower::new(None), 0)
}

// if a candidate sent a request vote, then this follower will vote for it
pub fn vote(machine: Raft<Follower>, candidate: String) -> Raft<Follower> {
  Raft::new(Follower::new(Some(candidate)), machine.term)
}

// another leader was successfully elected
pub fn complete_vote(machine: Raft<Follower>) -> Raft<Follower> {
  Raft::new(Follower::new(None), machine.term + 1)
}

// if the follower hit a timeout, it will become a candidate
pub fn election_timeout(machine: Raft<Follower>) -> Raft<Candidate> {
  Raft::<Candidate>::from(machine)
}

// if no majority was reached, then we just need to increment term and try again
pub fn failed_majority_timeout(machine: Raft<Candidate>) -> Raft<Candidate> {
  Raft::<Candidate>::from(machine)
}

// if another candidate reached a majority, then we go back down to a follower
pub fn other_leader_elected(machine: Raft<Candidate>) -> Raft<Follower> {
  Raft::<Follower>::from(machine)
}

// if a candidate successfully reached a majority, it is now a leader
pub fn elected(machine: Raft<Candidate>) -> Raft<Leader> {
  Raft::<Leader>::from(machine)
}
