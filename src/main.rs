use std::process;
use optagentnet::config::Config;
use optagentnet::pop::Population;
use rayon;
use std::time::Instant;
use optagentnet::ga::ga_step;

fn main() {

    // CONFIGURATION INITIALIZE
    let cfgfilename = String::from("oan.cfg"); 
    let cfg = Config::from_file(&cfgfilename)
        .unwrap_or_else(|err|{
            println!("Problem opening config file {} : {}",&cfgfilename,err);
            process::exit(1);
        });
             

    // THREADING (USING RAYON) 
    rayon::ThreadPoolBuilder::new().num_threads(cfg.threads as usize).build_global().unwrap();

    // LOAD INITIAL POPULATION
    let mut cur_pop =  Population::new_rand(&cfg)
        .unwrap_or_else(|err|{
            println!("Unable to create population of grids : {}",err);
            process::exit(1);
        });


    // TEST IF POP working (and time taken)
    //let start = Instant::now();
    //init_pop.execute_genomes_mt(&cfg);
    //let pop_sw = init_pop.get_sw_mt();
    //
    //let duration = start.elapsed();
    //println!("{:?}",init_pop);
    //println!("{:?}",pop_sw);
    //println!("Time : {:?}",duration);
    
    // START GA
    let start = Instant::now();
    for gen in 0..cfg.max_gen{
        cur_pop = ga_step(cur_pop,&cfg); // One step of genetic alorithm
        println!("{}",cur_pop.get_grid_at(0).get_cached_sw());
        println!("{}",gen);
        //println!("{:?}",cur_pop);
    }
    println!("{:?}",start.elapsed());

}
