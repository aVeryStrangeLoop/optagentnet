use std::process;
use optagentnet::config::Config;
use optagentnet::pop::Population;
use rayon;
use optagentnet::ga::ga_step;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::fs;

fn main() {
    
    let start_time= Instant::now();

    // INITIALIZE CONFIGURATION
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



    // INITIALISE OUTPUT FILES
    fs::create_dir_all("results").unwrap();    
    let mut summary = File::create("results/summary.csv").unwrap();
    summary.write(b"gen,best_sw\n").unwrap();

    

    // RUN GENETIC ALGROITHM
    for gen in 0..(cfg.max_gen+1){
        println!("Generation : {} out of {}",gen,cfg.max_gen);
        cur_pop = ga_step(cur_pop,&cfg);
        if gen%cfg.sv_every==0{
            save(&gen,&cur_pop,&summary);
        }  
    }

    // BRING THIS DOWN TO 5-10 minutes !!!
    println!("Completed run! ({:?})",start_time.elapsed());
}

pub fn save(gen: &u32,pop: &Population,mut sfile: &File) {
    sfile.write(format!("{},{}\n",gen,pop.grids[0].get_cached_sw()).as_bytes()).unwrap();
    let mut genres_file = File::create(format!("results/best_pop_{}.csv",gen)).unwrap();
    genres_file.write(pop.grids[0].seq_as_csv().as_bytes()).unwrap();
}
