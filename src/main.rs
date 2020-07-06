use std::process;
use optagentnet::config::Config;
use optagentnet::pop::Population;
use rayon;
use std::time::Instant;
fn main() {

    // Load configuration, exit if file not found and show config struct contents
    let cfgfilename = String::from("oan.cfg"); 
    
    let cfg = Config::from_file(&cfgfilename)
        .unwrap_or_else(|err|{
            println!("Problem opening config file {} : {}",&cfgfilename,err);
            process::exit(1);
        });
    println!("Loaded configuration : {}",cfg);
    

    // Initialise threads (only makes a difference if you use mt functions
    rayon::ThreadPoolBuilder::new().num_threads(cfg.threads as usize).build_global().unwrap();
    

    // Load initial random population of grids
    let init_pop =  Population::new_rand(&cfg)
        .unwrap_or_else(|err|{
            println!("Unable to create population of grids : {}",err);
            process::exit(1);
        });

    let start = Instant::now();
    // Calculate and return grid_sw's in population
    // get_sw is serial, get_sw_mt is parallel at population level (each grid gets a thread)
    let pop_sw = init_pop.get_sw();
    let duration = start.elapsed();
    println!("{:?}",init_pop);
    println!("{:?}",pop_sw);
    println!("Time : {:?}",duration);

}
