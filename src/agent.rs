use crate::config::Config;
use rand::Rng;
use std::fmt;

pub struct Agent{
    pub genome : Vec<i32>
}

impl Agent{
    // Create a new agent with a random genome
    pub fn new_rand(cfg: &Config) -> Agent {
        let mut genome = Vec::new();

        let mut rng = rand::thread_rng();

        for _ in 0..cfg.comp_cap{
            genome.push(rng.gen_range(1-(cfg.num_tasks as i32),cfg.num_tasks as i32)); 
        }
        Agent{genome:genome}
    }
}

impl fmt::Debug for Agent{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.genome)
    }
}
