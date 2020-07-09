use crate::config::Config;
use crate::agent::Agent;
use std::fmt;
//use rayon::prelude::*;

// RULE 0 : Never let anything above this API handle x,y coordinates
#[derive(Clone)]
pub struct Grid{
    pub agents : Vec<Agent>,
    pub size : u32, // size in each dimension
    pub sw : f64,
}

impl Grid{
    pub fn new_rand(cfg: &Config) -> Grid{
        let mut grid = Vec::new();
        let size = cfg.grid_size;
        for idx in 0..(cfg.grid_size * cfg.grid_size){
            let coords : (u32,u32) = (idx%cfg.grid_size,idx/cfg.grid_size);
            grid.push(Agent::new_rand(cfg,coords));
        }
        Grid{agents: grid,size:size,sw:0.0}
    }
 
    
    fn get_neighbors(&self, agent: &Agent) -> Vec<&Agent> {
        // Get agents around a particular agent (as references)
        let mut neighbors = Vec::new();

        // Given an agent, return Moore neighbors (agent.x = agent.pos.0, agent.y = agent.pos.1)
        // Neighbor types
        // 7       0(y+1)  1
        // 6(x-1)  ______  2(x+1)
        // 5       4(y-1)  3
        
        let (x,y) = agent.get_pos();

        // 0
        neighbors.push(&self.agents[self.xy_to_idx(if y+1<self.size {(x,y+1)} else {(x,0)}) as usize]); 
        
        // 1 
        neighbors.push(&self.agents[self.xy_to_idx(if x+1<self.size && y+1<self.size {(x+1,y+1)}
                                                   else if x+1<self.size && y+1>=self.size {(x+1,0)}
                                                   else if x+1>=self.size && y+1<self.size {(0,y+1)}
                                                   else {(0,0)}) as usize]);

        // 2
        neighbors.push(&self.agents[self.xy_to_idx(if x+1<self.size {(x+1,y)} else {(0,y)}) as usize]);

        // 3
        neighbors.push(&self.agents[self.xy_to_idx(if x+1<self.size && y>0 {(x+1,y-1)}
                                                   else if x+1<self.size && y<=0 {(x+1,self.size-1)}
                                                   else if x+1>=self.size && y>0 {(0,y-1)}
                                                   else {(0,self.size-1)}) as usize]);
        
        // 4
        neighbors.push(&self.agents[self.xy_to_idx(if y>0 {(x,y-1)} else {(x,self.size-1)}) as usize]);
        
        // 5
        neighbors.push(&self.agents[self.xy_to_idx(if x>0 && y>0 {(x-1,y-1)}
                                                   else if x<=0 && y>0 {(self.size-1,y-1)}
                                                   else if x>0 && y<=0 {(x-1,self.size-1)}
                                                   else {(self.size-1,self.size-1)}) as usize]);

        // 6 
        neighbors.push(&self.agents[self.xy_to_idx(if x>0 {(x-1,y)} else {(self.size-1,y)}) as usize]);   
        
        // 7
        neighbors.push(&self.agents[self.xy_to_idx(if x>0 &&  y+1<self.size {(x-1,y+1)}
                                                   else if x<=0 && y+1<self.size {(self.size-1,y+1)}
                                                   else if x>0 && y+1>=self.size {(x-1,0)}
                                                   else {(self.size-1,0)}) as usize]);


        neighbors
    }

    fn get_received_res(&self,agent: &Agent) -> Vec<f64>{
        let mut received = Vec::new();
        let neighbors = self.get_neighbors(agent);
        for neighbor in neighbors.iter() {
            // if received is empty, set it from the first neighbor
            if received.len()==0{
                received = neighbor.get_donated(); 
            } else{
                let donated_to_me = neighbor.get_donated();
                for idx in 0..received.len(){
                    received[idx] += donated_to_me[idx];
                }
            }
        }
        received
    }

    pub fn get_cached_sw(&self) -> f64 {
        self.sw
    }


    pub fn calculate_and_get_sw(&mut self) -> f64 {
        self.sw = self.agents.iter().map(|agent| agent.get_util()).sum();
        self.sw
    }
        
    pub fn execute_genomes(&mut self,cfg: &Config) {
        self.agents.iter_mut().for_each(|agent| agent.execute_genome(cfg));
        for idx in 0..self.agents.len(){
            let received = self.get_received_res(&self.agents[idx]);
            self.agents[idx].add_resource(received);
        }
    }

    pub fn mutate(&mut self, cfg: &Config) {
       self.agents.iter_mut().for_each(|agent| agent.mutate(cfg)); 
    }
   
    // UNCOMMENT TO ENABLE GRID LEVEL MULTITHREADING (Not recommended except for complicated utility functions) 
    // Multithread implementation of get_sw. Each agent gets a thread
    // pub fn get_sw_mt(&self) -> f64{
    //    self.agents.par_iter().map(|agent| agent.get_util()).sum()
    // }

    // pub fn execute_genomes_mt(&mut self, cfg: &Config) {
    //    self.agents.par_iter_mut().for_each(|agent| agent.execute_genome(cfg));
    //    // DO DONATIONS HERE !!
    // }


    fn xy_to_idx(&self, xy: (u32,u32)) -> u32 {
        // Convert xy position to grid index
        let idx = xy.1 * self.size + xy.0;
        idx
    }

    pub fn get_agent_genome_at(&self,idx: usize) -> Vec<i32> {
        self.agents[idx].get_genome()
    }

    pub fn set_agent_genome_at(&mut self, idx: usize, genome:Vec<i32>){
        self.agents[idx].set_genome(genome);
    }

}

impl fmt::Debug for Grid{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        write!(f,"\n{:?}\n",self.agents)
    }

}
