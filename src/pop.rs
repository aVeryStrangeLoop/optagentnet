use crate::config::Config;
use crate::grid::Grid;
use std::error::Error;
use std::fmt;

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
}

impl fmt::Debug for Population{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.grids)
    }
}
