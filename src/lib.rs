use std::time::{Duration,Instant};
use rand::Rng;
#[derive(Debug, Clone, PartialEq)]
pub enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug)]
pub struct Message {
    pub from: u64,
    pub to: u64,
    pub msg_type: MessageType,
}

#[derive(Debug)]
pub enum MessageType {
    Heartbeat,
    RequestVote,
    VoteReponse(bool),
}

pub struct Node {
    id: u64,
    state: NodeState,
    term: u64,
    peers: Vec<u64>,
    last_heartbeat:Instant,
    election_timeout:Duration,
}

impl Node {
    pub fn new(id: u64, peers: Vec<u64>) -> Self {
        Node {
            id,
            state: NodeState::Follower,
            term: 0,
            peers,
            last_heartbeat:Instant::now(),
            election_timeout:Self::random_timeout()
        }
    }

    pub fn status(&self) {
        println!("Node {} is {:?} at term {}", self.id, self.state, self.term);
    }
    fn random_timeout()->Duration{
       return  Duration::from_millis(rand::thread_rng().gen_range(100..300));
    }
    pub fn check_timeout(&mut self){
        if self.last_heartbeat.elapsed()>=self.election_timeout{
            self.start_election();
        }
    }
    fn start_election(&mut self){
        self.state=NodeState::Candidate;
        self.term+=1;
        self.last_heartbeat=Instant::now();
        println!("Node {} starting election for the term {}",self.id,self.term);

    }
    pub fn recieve_heartbeat(&mut self){
        self.last_heartbeat=Instant::now();
        if self.state==NodeState::Candidate{
            self.state=NodeState::Follower;
        }
    }
}