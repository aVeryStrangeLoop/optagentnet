use std::fs;
use std::error::Error;
use std::fmt;

// CONFIGURATION INITIALISATION
pub struct Config {
    // Structural and environmental parameters
    pub grid_size : u32, // Grid size
    pub num_tasks : u32, // Number of tasks in env
    pub comp_cap  : u32, // Computational capacity
    
    // UF and agent parameters
    pub exp_eff   : f64, // Export efficacy
    pub uf_param1 : f64, // Compounding coefficient
    

    // GA paramters
    pub gpop_size  : u32,// GA Population size
    pub gmut_prob  : f64,// Mutation probability
    pub gcrs_prob  : f64,// Crossover probability
    

    // Task to info
    pub tti_alpha : f64, // Task to info alpha
    pub tti_beta  : f64, // Task to info beta

    // RUN properties
    pub max_gen   : u32, // Total number of gens to run for
    pub sv_every  : u32, // Save every how many gens

}

impl Config{

    // Initialize config from file
    pub fn from_file(cfgfilename: &String) -> Result<Config,Box<dyn Error>>{
        
        let cfg_content = fs::read_to_string(cfgfilename)?; // The ? here returns an error if file cannot be opened
        
        let mut config = Config::default();

        for line in cfg_content.lines(){
            let as_vec : Vec<&str> = line.split(",").collect();
            
            match &as_vec[0][..]{
                "grid_size" => {config.grid_size = as_vec[1].parse::<u32>().unwrap()},
                "num_tasks" => {config.num_tasks = as_vec[1].parse::<u32>().unwrap()},
                "comp_cap"  => {config.comp_cap  = as_vec[1].parse::<u32>().unwrap()},
                "exp_eff"   => {config.exp_eff   = as_vec[1].parse::<f64>().unwrap()},
                "uf_param1" => {config.uf_param1 = as_vec[1].parse::<f64>().unwrap()},
                "gpop_size" => {config.gpop_size = as_vec[1].parse::<u32>().unwrap()},
                "gmut_prob" => {config.gmut_prob = as_vec[1].parse::<f64>().unwrap()},
                "gcrs_prob" => {config.gcrs_prob = as_vec[1].parse::<f64>().unwrap()},
                "tti_alpha" => {config.tti_alpha = as_vec[1].parse::<f64>().unwrap()},
                "tti_beta"  => {config.tti_beta  = as_vec[1].parse::<f64>().unwrap()},
                "max_gen"   => {config.max_gen   = as_vec[1].parse::<u32>().unwrap()},
                "sv_every"  => {config.sv_every  = as_vec[1].parse::<u32>().unwrap()},
                _=>{},
            }
        }

        Ok(config)
    }
}

impl Default for Config{
    fn default() -> Config {
        Config{grid_size:0,
            num_tasks: 0,
            comp_cap: 0,
            exp_eff: 0.0,
            uf_param1: 0.0,
            gpop_size: 0,
            gmut_prob: 0.0,
            gcrs_prob: 0.0,
            tti_alpha: 0.0,
            tti_beta: 0.0,
            max_gen: 0,
            sv_every: 0
        }
    }
}

impl fmt::Display for Config{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{},{},{},{},{},{},{},{},{},{},{},{}",
            self.grid_size,
            self.num_tasks,
            self.comp_cap,
            self.exp_eff,
            self.uf_param1,
            self.gpop_size,
            self.gmut_prob,
            self.gcrs_prob,
            self.tti_alpha,
            self.tti_beta,
            self.max_gen,
            self.sv_every)
    }
}

