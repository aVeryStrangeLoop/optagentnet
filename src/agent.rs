use crate::config::Config;
use rand::Rng;
use std::fmt;
use crate::uf::utility_function;

pub struct Agent{
    genome : Vec<i32>,
    position : (u32,u32), //x,y position of agent 
    res : Vec<f64>,
}

impl Agent{
    // Create a new agent with a random genome
    pub fn new_rand(cfg: &Config, coords: (u32,u32)) -> Agent {
        let mut genome = Vec::new();
        let mut res = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..cfg.comp_cap{
            genome.push(rng.gen_range(1-(cfg.num_tasks as i32),cfg.num_tasks as i32)); 
        }
        for _ in 0..cfg.num_tasks{
            res.push(0.0);
        }
        Agent{genome:genome,position:coords,res:res}
    }

    pub fn get_pos(&self) -> (u32,u32) {
        self.position
    }

    pub fn get_util(&self) -> f64 {
        utility_function(&self.res)
    }
}

impl fmt::Debug for Agent{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.genome)
    }
}
