use crate::config::Config;
use crate::grid::Grid;
use std::error::Error;
use std::fmt;
use rayon::prelude::*;

pub struct Population{
    pub grids : Vec<Grid>
}

impl Population{
    pub fn new_rand(cfg: &Config) -> Result<Population,Box<dyn Error>>{
        let mut pop = Vec::new();

        for _ in 0..cfg.gpop_size{
            pop.push(Grid::new_rand(cfg));
        }
        Ok(Population{grids: pop})
    }

    pub fn get_sw(&self) -> Vec<f64> { // Serial
        self.grids.iter().map(|grid| grid.get_sw()).collect()
    }

    pub fn get_sw_mt(&self) -> Vec<f64> { // Each grid gets a thread
        self.grids.par_iter().map(|grid| grid.get_sw()).collect() // Change get_sw to get_sw_mt for each agent to get a thread. Not recommended unles resvec -> utility is an expensive calculation.
    }
}

impl fmt::Debug for Population{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.grids)
    }
}
