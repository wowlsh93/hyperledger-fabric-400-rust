use crate::fabric::core::Fabric;

pub fn client_sdk() -> ClientSdk {

    return ClientSdk{ f :  Fabric::new() }
}

pub struct ClientSdk {
    f : Fabric
}

impl ClientSdk {

    pub fn start_fabric (&self) {
        self.f.start();
    }

    pub fn write_trans(&self, k : &str, v : &str) -> &str {
        let rwset = self.f.write_transaction(k,v);
        "fail"
    }

}
