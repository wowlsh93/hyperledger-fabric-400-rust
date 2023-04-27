use std::{io, thread, time};
use std::time::Duration;
use crossbeam::crossbeam_channel::{never, unbounded,Select,RecvError, Sender, Receiver};
use crossbeam::crossbeam_channel::select;
use std::thread::{sleep, Thread};
use std::sync::Arc;

pub struct RWSet {
    msp : String,
    key : String,
    value : String,
    peers_map :  Vec<String>
}

pub struct Transaction {
    msp : String,
    key : String,
    value : String
}

pub struct _Transaction {
    key : String,
    value : String
}

pub struct Peer {
    // msp : String;
    pub sender : Sender<String>,
    pub receiver : Receiver<String>
}


impl Peer {

    pub fn new() -> Peer {

        let c1 : (Sender<String>, Receiver<String>) = unbounded();
        let c2 : (Sender<String>, Receiver<String>) = unbounded();

        let p = Peer { sender : c1.0,  receiver: c2.1 };

        let ch1_receiver= c1.1;
        let ch2_sender = c2.0;

        thread::spawn(move || {

            loop {
                select! {
                        recv(ch1_receiver) -> msg => {
                                 if msg == Err(RecvError) {
                                   println!("recv error");
                                 }else{
                                    ch2_sender.send(String::from("rwset-data")).unwrap();
                                 }}
                                ,
                        default(Duration::from_secs(3)) => {
                            println!("timed out");
                            break;
                        }
                    }
            }

        });

        return p;
    }

    pub fn send_transaction(&self, msg : String) -> String {
        self.sender.send(msg).unwrap();
        return self.receiver.recv().unwrap();
    }
}

pub struct Fabric {

    pub endorser1 : Peer,
    pub endorser2 : Peer,

    pub MSP_org1 : String
}

impl Fabric {

    pub fn new() -> Fabric {
        Fabric{endorser1 : Peer::new(), endorser2 : Peer::new(), MSP_org1 : String::from("org1")}
    }

    pub fn start(&self) {

    }

    pub fn send_transaction(&self, key: &str, value :  &str) ->  Result<(String,String), io::Error> {

        // let t = Transaction{
        //     msp: self.MSP_org1,
        //     key: key,
        //     value: value
        // };

        let s1 = self.endorser1.send_transaction(String::from("hi"));
        let s2 = self.endorser2.send_transaction(String::from("hi"));

        return Ok((s1, s2));
    }
}
