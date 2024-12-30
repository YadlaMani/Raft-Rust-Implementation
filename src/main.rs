use raft_consensus::Node;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let mut node=Node::new(1,vec![2,3]);
    for _ in 0..5{
        node.check_timeout();
        sleep(Duration::from_millis(200));

    }
}
