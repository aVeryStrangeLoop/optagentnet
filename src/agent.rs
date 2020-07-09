use crate::config::Config;
use rand::Rng;
use std::fmt;
use crate::uf::utility_function;
#[derive(Clone)]
pub struct Agent{
    genome : Vec<i32>,
    position : (u32,u32), //x,y position of agent 
    res : Vec<f64>,
    donate: Vec<f64>
}

impl Agent{
    // Create a new agent with a random genome
    pub fn new_rand(cfg: &Config, coords: (u32,u32)) -> Agent {
        let mut genome = Vec::new();
        let mut res = Vec::new();
        let mut donate = Vec::new();

        let mut rng = rand::thread_rng();

        for _ in 0..cfg.comp_cap{
            genome.push(rng.gen_range(1-(cfg.num_tasks as i32),cfg.num_tasks as i32)); 
        }
        for _ in 0..cfg.num_tasks{
            res.push(0.0);
            donate.push(0.0);
        }
        Agent{genome:genome,position:coords,res:res,donate:donate}
    }

    pub fn get_pos(&self) -> (u32,u32) {
        self.position
    }

    pub fn get_util(&self,cfg:&Config) -> f64 {
        utility_function(&self.res,cfg)
    }

    pub fn execute_genome(&mut self,cfg: &Config) {
        // Process positive and add to resource vector
        for task in self.res.iter_mut(){
            *task = 0.0
        }

        for task in self.donate.iter_mut(){
            *task = 0.0
        }
        for val in self.genome.iter(){
            if *val>0 {
                self.res[*val as usize] += cfg.tti_alpha * (*val as f64) + cfg.tti_beta;
            } 
        }

        for val in self.genome.iter(){
            if *val < 0{
                let donated = self.res[(-*val) as usize] * cfg.exp_eff;
                self.res[(-*val) as usize] -= donated;
                self.donate[(-*val) as usize] += donated/8.0; // As there are eight neighbors (no division takes place later)
            }
        }
    }

    pub fn mutate(&mut self,cfg:&Config){
        let mut rng = rand::thread_rng();
        for idx in 0..self.genome.len(){
            let rn : f64 = rng.gen();
            if rn < cfg.gmut_prob { self.genome[idx] = rng.gen_range(1-(cfg.num_tasks as i32),cfg.num_tasks as i32) }
        }
    }

    pub fn add_resource(&mut self,to_add_res: Vec<f64>){
        for idx in 0..to_add_res.len(){
            self.res[idx] += to_add_res[idx];
        }
    }

    pub fn get_donated(&self) -> Vec<f64> {
        self.donate.clone()
    }

    pub fn get_genome(&self) -> Vec<i32> {
        self.genome.clone()
    }

    pub fn set_genome(&mut self,vec:Vec<i32>) {
        self.genome = vec;
    }
}

impl fmt::Debug for Agent{
    fn fmt(& self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.genome)
    }
}
