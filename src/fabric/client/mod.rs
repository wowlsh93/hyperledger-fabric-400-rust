use std::io;
use super::core::Fabric;

pub struct ClientSDK {
    f : Fabric
}

impl ClientSDK {

    pub fn new() -> ClientSDK {
        return ClientSDK{ f :  Fabric::new() }
    }

    pub fn start_new() -> ClientSDK {
        let sdk = ClientSDK{ f :  Fabric::new() };
        sdk.start_fabric();
        return sdk;
    }

    pub fn start_fabric (&self) {
        self.f.start();
    }

    pub fn send_transaction(&self, k : &str, v : &str) -> Result<(String,String), io::Error>  {
        return self.f.send_transaction(k,v);
    }

}
