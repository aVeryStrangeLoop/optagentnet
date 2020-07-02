use crate::config::Config;
use crate::agent::Agent;
use std::fmt;

pub struct Grid{
    pub agents : Vec<Agent>
}

impl Grid{
    pub fn new(cfg: &Config) -> Grid{
        let mut grid = Vec::new();
        
        for _ in 0..(cfg.grid_size * cfg.grid_size){
            grid.push(Agent::new_rand(cfg));
        }
        Grid{agents: grid}
    }
}

impl fmt::Debug for Grid{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        write!(f,"\n{:?}\n",self.agents)
    }
}
