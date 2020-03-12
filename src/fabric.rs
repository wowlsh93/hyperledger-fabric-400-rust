

mod core;

pub fn client_sdk() -> ClientSdk {

    return ClientSdk{ f : core::Fabric::new() }
}

pub struct ClientSdk {
    f : core::Fabric
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
