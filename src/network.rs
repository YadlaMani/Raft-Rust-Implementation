use tokio::sync::mpsc::{self,Receiver,Sender};
use serde::{Deserialize,Serialize};
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct NetworkMessage{
    pub from:u64,
    pub to:u64,
    pub term:u64,
    pub payload:MessagePayload,
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum MessagePayload{
    Heartbeat,
    RequestVote,
    VoteGranted,
}
pub struct Network{
    node_id:u64,
    tx:Sender<NetworkMessage>,
    rx:Receiver<NetworkMessage>
}
impl Network{
    pub fn new(node_id:u64)->(Self,Sender<NetworkMessage){
        let (tx,rx)=mpsc::channel(100);
        (Self {node_id,tx:tx.clone(),rx},tx)
    }
    pub async fn send(&self,msg:NetworkMessage){
        if let Err(e)=self.tx.send(msg).await{
            println!("Failed to send message:{}",e);
        }
    }
    pub async fn recieve(&mut self)->Option<NetworkMessage>{
        self.rx.recv().await
    }
}