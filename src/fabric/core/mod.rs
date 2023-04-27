use std::{thread, time};
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
    pub s1 : Sender<String>,
    pub r2 : Receiver<String>
}


impl Peer {

    pub fn new() -> Peer {

        let c1 : (Sender<String>, Receiver<String>) = unbounded();
        let c2 : (Sender<String>, Receiver<String>) = unbounded();

        let p = Peer { s1 : c1.0,  r2: c2.1 };

        let r1= c1.1;
        let s2 = c2.0;

        thread::spawn(move || {

            loop {
                select! {
                        recv(r1) -> msg => {
                                 if msg == Err(RecvError) {
                                   println!("err_r1");
                                 }else{
                                    //println!("{}", msg.unwrap());
                                    s2.send(String::from("bye")).unwrap();
                                 }}
                                ,
                        default(Duration::from_secs(3)) => println!("timed out")
                    }
            }

        });

        return p;
    }

    pub fn add_trans(&self, s : String) -> String {
        self.s1.send(s).unwrap();
        let a = self.r2.recv().unwrap();

        //println!("{}", a.as_str());
        a

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

    pub fn write_transaction(&self, key: &str, value :  &str) -> ((String,String)) {

        // let t = Transaction{
        //     msp: self.MSP_org1,
        //     key: key,
        //     value: value
        // };

        let s1 = self.endorser1.add_trans(String::from("hi"));
        let s2 = self.endorser2.add_trans(String::from("hi"));

        return (s1, s2);
    }
}
