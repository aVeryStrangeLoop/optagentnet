use std::process;

use optagentnet::config::Config;

fn main() {

    // Load configuration, exit if file not found and show config struct contents
    let cfgfilename = String::from("oan.cfg"); 
    let config = Config::from_file(&cfgfilename)
        .unwrap_or_else(|err|{
            println!("Problem opening config file {} : {}",&cfgfilename,err);
            process::exit(1);
        });

    println!("Loaded configuration : {}",config);


}
