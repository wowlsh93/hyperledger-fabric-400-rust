use std::{thread, time};
use std::time::Duration;
use crossbeam::crossbeam_channel::{never, unbounded,Select,RecvError, Sender, Receiver};
use crossbeam::crossbeam_channel::select;
use std::thread::{sleep, Thread};

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
    pub c1 : (Sender<String>, Receiver<String>),
    pub c2 : (Sender<String>, Receiver<String>)
}


impl Peer {

    pub fn new() -> Peer {

       return Peer {c1 : unbounded(), c2 : unbounded() };
    }

    pub fn start(&self) {

        thread::spawn(move || {
            loop {
                    select! {
                        recv(self.c1.1) -> msg => {
                                 if msg == Err(RecvError) {
                                   println!("err_r1");
                                 }else{
                                    println!("{}", msg.unwrap());
                                    self.c2.0.send(String::from("20")).unwrap();
                                 }}
                                ,
                        default(Duration::from_secs(3)) => println!("timed out")
                    }
            }}
        );

    }

    pub fn add_trans(&self, s : String) -> String {
        self.c1.0.send(s).unwrap();
        let a = self.c2.1.recv().unwrap();

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

        self.endorser1.start();
        self.endorser2.start();

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
