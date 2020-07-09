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

    pub fn new_empty() -> Result<Population,Box<dyn Error>>{
        let pop = Vec::new();
        Ok(Population{grids:pop})
    }

    pub fn push(&mut self,grid: Grid){
        self.grids.push(grid);
    }

    pub fn get_grid_at(&self,idx: usize) -> Grid {
        self.grids[idx].clone()
    }

    pub fn sort_rev_by(&mut self) {
        // Sort self based on the key specified by vec 
        // Eg- Pop{[g1,g2,g3]} with vec{0.1,0.3,0.2} gets sorted as
        // Pop{[g2,g3,g1]}
        self.grids.sort_by(|g1,g2| g1.get_cached_sw().partial_cmp(&g2.get_cached_sw()).unwrap());
        self.grids.reverse();
    }

    pub fn get_sw(&mut self) -> Vec<f64> { // Serial
        self.grids.iter_mut().map(|grid| grid.calculate_and_get_sw()).collect()
    }
    
    pub fn execute_genomes(&mut self, cfg: &Config) {
        self.grids.iter_mut().for_each(|grid| grid.execute_genomes(cfg));
    }

    pub fn get_sw_mt(&mut self) -> Vec<f64> { // Each grid gets a thread
        self.grids.par_iter_mut().map(|grid| grid.calculate_and_get_sw()).collect() // Change get_sw to get_sw_mt for each agent to get a thread. Not recommended unles resvec -> utility is an expensive calculation.
    }

    pub fn execute_genomes_mt(&mut self, cfg: &Config) { // Each grid gets a thread
        self.grids.par_iter_mut().for_each(|grid| grid.execute_genomes(cfg));  //  Change execute_genomes to execute_genomes_mt for each agent to get a thread.
    }
}

impl fmt::Debug for Population{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.grids)
    }
}
