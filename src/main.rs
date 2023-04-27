mod fabric;

use std::thread::sleep;
use std::time::Duration;
use stopwatch::{Stopwatch};
use fabric::client::ClientSDK;

fn main() {

    println!("act - 1");

    let  sdk: ClientSDK = ClientSDK::start_new();
    let sw = Stopwatch::start_new();

    println!("act - 2");


    for i in 0 ..=10 {
        match  sdk.send_transaction(&i.to_string() , &i.to_string()) {
            Ok(rwset) => println!("{}-{}", rwset.0, rwset.1),
            Err(err) => println!("{}", err)
        }
    }

    // println!("act - 3");

    // loop {
    //     let result = sdk.get_rans("1000000");
    //     if result != "" {
    //         break;
    //     }
    // }

    // println!("act - 4");
    println!("benchmark took {}ms", sw.elapsed_ms());
    
    sleep(Duration::from_secs(10));

}

