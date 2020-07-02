use crate::config::Config;
use crate::agent::Agent;
use std::fmt;

// RULE 0 : Never let anything above this API handle x,y coordinates
pub struct Grid{
    pub agents : Vec<Agent>,
    pub size : u32, // size in each dimension
}

impl Grid{
    pub fn new_rand(cfg: &Config) -> Grid{
        let mut grid = Vec::new();
        let size = cfg.grid_size;
        for idx in 0..(cfg.grid_size * cfg.grid_size){
            let coords : (u32,u32) = (idx%cfg.grid_size,idx/cfg.grid_size);
            grid.push(Agent::new_rand(cfg,coords));
        }
        Grid{agents: grid,size:size}
    }
 
    
    pub fn get_neighbors(&self, agent: &Agent) -> Vec<&Agent> {
        // Get agents around a particular agent (as references)
        let mut neighbors = Vec::new();

        // Given an agent, return Moore neighbors (agent.x = agent.pos.0, agent.y = agent.pos.1)
        // Neighbor types
        // 7       0(y+1)  1
        // 6(x-1)  ______  2(x+1)
        // 5       4(y-1)  3
        
        let x = agent.position.0;
        let y = agent.position.1;

        // 0
        neighbors.push(&self.agents[self.xy_to_idx(if y+1<self.size {(x,y+1)} else {(x,0)}) as usize]); 
        
        // 1 
        neighbors.push(&self.agents[self.xy_to_idx(if (x+1<self.size && y+1<self.size) {(x+1,y+1)}
                                                   else if (x+1<self.size && y+1>=self.size) {(x+1,0)}
                                                   else if (x+1>=self.size && y+1<self.size) {(0,y+1)}
                                                   else {(0,0)}) as usize]);

        // 2
        neighbors.push(&self.agents[self.xy_to_idx(if x+1<self.size {(x+1,y)} else {(0,y)}) as usize]);

        // 3
        neighbors.push(&self.agents[self.xy_to_idx(if (x+1<self.size && y>0) {(x+1,y-1)}
                                                   else if (x+1<self.size && y<=0) {(x+1,self.size-1)}
                                                   else if (x+1>=self.size && y>0) {(0,y-1)}
                                                   else {(0,self.size-1)}) as usize]);
        
        // 4
        neighbors.push(&self.agents[self.xy_to_idx(if y>0 {(x,y-1)} else {(x,self.size-1)}) as usize]);
        
        // 5
        neighbors.push(&self.agents[self.xy_to_idx(if (x>0 && y>0) {(x-1,y-1)}
                                                   else if (x<=0 && y>0) {(self.size-1,y-1)}
                                                   else if (x>0 && y<=0) {(x-1,self.size-1)}
                                                   else {(self.size-1,self.size-1)}) as usize]);

        // 6 
        neighbors.push(&self.agents[self.xy_to_idx(if x>0 {(x-1,y)} else {(self.size-1,y)}) as usize]);   
        
        // 7
        neighbors.push(&self.agents[self.xy_to_idx(if (x>0 &&  y+1<self.size) {(x-1,y+1)}
                                                   else if (x<=0 && y+1<self.size) {(self.size-1,y+1)}
                                                   else if (x>0 && y+1>=self.size) {(x-1,0)}
                                                   else {(self.size-1,0)}) as usize]);


        neighbors
    }

    fn idx_to_xy(&self, idx: u32) -> (u32,u32) {
        // Convert vector index to xy position
        let x = idx % self.size;
        let y = idx / self.size; 
        (x,y)
    }


    fn xy_to_idx(&self, xy: (u32,u32)) -> u32 {
        // Convert xy position to grid index
        let idx = xy.1 * self.size + xy.0;
        idx
    }
}

impl fmt::Debug for Grid{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        write!(f,"\n{:?}\n",self.agents)
    }

}
