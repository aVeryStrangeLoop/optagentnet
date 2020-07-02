use std::process;
use optagentnet::config::Config;
use optagentnet::pop::Population;

fn main() {

    // Load configuration, exit if file not found and show config struct contents
    let cfgfilename = String::from("oan.cfg"); 
    let cfg = Config::from_file(&cfgfilename)
        .unwrap_or_else(|err|{
            println!("Problem opening config file {} : {}",&cfgfilename,err);
            process::exit(1);
        });
    println!("Loaded configuration : {}",cfg);


    // Load initial random population of grids
    let init_pop =  Population::new_rand(&cfg)
        .unwrap_or_else(|err|{
            println!("Unable to create population of grids : {}",err);
            process::exit(1);
        });

    // Calculate SW of each grid in the population


}
