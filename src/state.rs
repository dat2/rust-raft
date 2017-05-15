#[derive(Debug,Clone)]
pub struct RaftState {
  state: StateEnum
}

impl RaftState {
  pub fn new() -> RaftState {
    RaftState { state: StateEnum::Follower(StateMachine::init()) }
  }

  pub fn vote(&mut self, candidate: String) {
    if let StateEnum::Follower(m) = self.state.clone() {
      self.state = StateEnum::Follower(m.vote(candidate));
    }
  }
}

#[derive(Debug,Clone)]
pub enum StateEnum {
  Follower(StateMachine<Follower>),
  Candidate(StateMachine<Candidate>),
  Leader(StateMachine<Leader>)
}

#[derive(Debug,Clone)]
pub struct StateMachine<S> {
  pub state: S,
  pub term: usize,
}

impl<S> StateMachine<S> {
  fn new(state: S, term: usize) -> StateMachine<S> {
    StateMachine {
      state: state,
      term: term,
    }
  }
}

impl StateMachine<Follower> {
  // everyone starts off as a follower
  pub fn init() -> StateMachine<Follower> {
    StateMachine::new(Follower::new(None), 0)
  }

  // if a candidate sent a request vote, then this follower will vote for it
  pub fn vote(self, candidate: String) -> StateMachine<Follower> {
    StateMachine::new(Follower::new(Some(candidate)), self.term)
  }

  // another leader was successfully elected
  pub fn complete_vote(self) -> StateMachine<Follower> {
    StateMachine::new(Follower::new(None), self.term + 1)
  }

  // if the follower hit a timeout, it will become a candidate
  pub fn election_timeout(self) -> StateMachine<Candidate> {
    StateMachine::<Candidate>::from(self)
  }
}

impl StateMachine<Candidate> {
  // if no majority was reached, then we just need to increment term and try again
  pub fn failed_majority_timeout(self) -> StateMachine<Candidate> {
    StateMachine::<Candidate>::from(self)
  }

  // if another candidate reached a majority, then we go back down to a follower
  pub fn other_leader_elected(self) -> StateMachine<Follower> {
    StateMachine::<Follower>::from(self)
  }

  // if a candidate successfully reached a majority, it is now a leader
  pub fn elected(self) -> StateMachine<Leader> {
    StateMachine::<Leader>::from(self)
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
impl From<StateMachine<Follower>> for StateMachine<Candidate> {
  fn from(val: StateMachine<Follower>) -> StateMachine<Candidate> {
    StateMachine::new(Candidate, val.term + 1)
  }
}

// candidate to follower (eg. another leader has been elected)
impl From<StateMachine<Candidate>> for StateMachine<Follower> {
  fn from(val: StateMachine<Candidate>) -> StateMachine<Follower> {
    StateMachine::new(Follower::new(None), val.term + 1)
  }
}

// candidate to leader (eg. candidate successfully hit a majority)
impl From<StateMachine<Candidate>> for StateMachine<Leader> {
  fn from(val: StateMachine<Candidate>) -> StateMachine<Leader> {
    StateMachine::new(Leader, val.term + 1)
  }
}
