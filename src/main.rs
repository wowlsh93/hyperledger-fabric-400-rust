mod fabric;

use stopwatch::{Stopwatch};
use fabric::client::ClientSDK;

fn main() {

    println!("act - 1");

    let  sdk: ClientSDK = ClientSDK::start_new();
    let sw = Stopwatch::start_new();

    println!("act - 2");


    for i in 0 ..1000000 {
        sdk.write_trans(&i.to_string() , &i.to_string());
    }

    println!("act - 3");
    //
    // loop {
    //     let result = sdk.get_rans("1000000");
    //     if result != "" {
    //         break;
    //     }
    // }

    println!("act - 4");
    println!("benchmark took {}ms", sw.elapsed_ms());

}

