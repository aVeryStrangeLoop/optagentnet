use std::process;

use optagentnet::Config;

fn main() {

    // Load configuration and exit if file not found
    let cfgfilename = String::from("oan.cfg"); 
    let config = Config::from_file(&cfgfilename)
        .unwrap_or_else(|err|{
            println!("Problem opening config file {} : {}",&cfgfilename,err);
            process::exit(1);
        });

    println!("{}",config);


}
