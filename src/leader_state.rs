#[derive(Debug,Clone)]
pub struct LeaderState<S> {
  pub state: S,
  pub term: usize,
}

impl<S> LeaderState<S> {
  fn new(state: S, term: usize) -> LeaderState<S> {
    LeaderState {
      state: state,
      term: term,
    }
  }
}

#[derive(Debug,Clone)]
pub struct Follower {
  voted_for: Option<String>,
}

impl Follower {
  fn new(voted_for: Option<String>) -> Follower {
    Follower { voted_for: voted_for }
  }
}

#[derive(Debug,Clone)]
pub struct Candidate;

#[derive(Debug,Clone)]
pub struct Leader;

// follower to candidate
impl From<LeaderState<Follower>> for LeaderState<Candidate> {
  fn from(val: LeaderState<Follower>) -> LeaderState<Candidate> {
    LeaderState::new(Candidate, val.term + 1)
  }
}

// candidate to follower (eg. another leader has been elected)
impl From<LeaderState<Candidate>> for LeaderState<Follower> {
  fn from(val: LeaderState<Candidate>) -> LeaderState<Follower> {
    LeaderState::new(Follower::new(None), val.term + 1)
  }
}

// candidate to leader (eg. candidate successfully hit a majority)
impl From<LeaderState<Candidate>> for LeaderState<Leader> {
  fn from(val: LeaderState<Candidate>) -> LeaderState<Leader> {
    LeaderState::new(Leader, val.term + 1)
  }
}

// everyone starts off as a follower
pub fn init_state() -> LeaderState<Follower> {
  LeaderState::new(Follower::new(None), 0)
}

// if a candidate sent a request vote, then this follower will vote for it
pub fn vote(machine: LeaderState<Follower>, candidate: String) -> LeaderState<Follower> {
  LeaderState::new(Follower::new(Some(candidate)), machine.term)
}

// another leader was successfully elected
pub fn complete_vote(machine: LeaderState<Follower>) -> LeaderState<Follower> {
  LeaderState::new(Follower::new(None), machine.term + 1)
}

// if the follower hit a timeout, it will become a candidate
pub fn election_timeout(machine: LeaderState<Follower>) -> LeaderState<Candidate> {
  LeaderState::<Candidate>::from(machine)
}

// if no majority was reached, then we just need to increment term and try again
pub fn failed_majority_timeout(machine: LeaderState<Candidate>) -> LeaderState<Candidate> {
  LeaderState::<Candidate>::from(machine)
}

// if another candidate reached a majority, then we go back down to a follower
pub fn other_leader_elected(machine: LeaderState<Candidate>) -> LeaderState<Follower> {
  LeaderState::<Follower>::from(machine)
}

// if a candidate successfully reached a majority, it is now a leader
pub fn elected(machine: LeaderState<Candidate>) -> LeaderState<Leader> {
  LeaderState::<Leader>::from(machine)
}
