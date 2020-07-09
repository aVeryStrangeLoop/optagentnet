use crate::pop::Population;
use crate::grid::Grid;
use crate::config::Config;
use rand::Rng;
use std::process;

pub fn ga_step(mut pop: Population,cfg: &Config) -> Population {
    // Execute pop genomes to
    pop.execute_genomes_mt(&cfg);    
    // Get Population and sw vector sorted
    let mut sw = pop.get_sw_mt();
    pop.sort_rev_by(); // sort population by sw (and reverse)
    sw.sort_by(|a,b| a.partial_cmp(b).unwrap());
    sw.reverse();
    // Normalise sw vector to get as probabilities
    sw = normalise(sw);

    

    // Create the new population to return and add the two fittest grids to it
    let mut new_pop = Population::new_empty()
        .unwrap_or_else(|err| {
            println!("Unable to create empty population: {}",err);
            process::exit(1);
        });
    new_pop.push(pop.get_grid_at(0)); // Get cloned best grid
    new_pop.push(pop.get_grid_at(1)); // Get clone second best grid

    for _ in (2..cfg.gpop_size).step_by(2) {
        // MOST EXPENSIVE 1 (TODO)
        let mut g1 = pop.get_grid_at(idx_from_prob_dist(&sw)); // Get cloned
        let mut g2 = pop.get_grid_at(idx_from_prob_dist(&sw)); // Get cloned
        // with gcrs_prob, crossover
        if true_with_prob(cfg.gcrs_prob){
            let new_grids = crossover(g1,g2);
            g1 = new_grids.0;
            g2 = new_grids.1;
        }
        
        // MOST EXPENSIVE 2 (TODO)
        // Mutate in place
        g1.mutate(&cfg);
        g2.mutate(&cfg);
        new_pop.push(g1);
        new_pop.push(g2);
    } 


    return new_pop
}

pub fn normalise(vec: Vec<f64>) -> Vec<f64> {
    let sum: f64 = vec.iter().sum();
    vec.iter().map(|x| x/sum).collect()
}

pub fn true_with_prob(prob: f64) -> bool {
    let rn = rand::thread_rng().gen();
    if prob>rn { true } else { false }
}

pub fn idx_from_prob_dist(probs: &Vec<f64>) -> usize {
    let rn : f64 = rand::thread_rng().gen();
    let mut sum = 0.0;
    for idx in 0..probs.len(){
        sum += probs[idx];
        if rn <= sum {
            return idx;} 
    }
    1
}

pub fn crossover(mut g1: Grid,mut g2: Grid) -> (Grid,Grid){
    // Get a random index for agent
    let rn: usize = rand::thread_rng().gen_range(0,g1.agents.len());
    let mut genome1 = g1.get_agent_genome_at(rn); 
    let mut genome2 = g2.get_agent_genome_at(rn); 
    let new_genomes = single_site_cross(genome1,genome2); 
    genome1 = new_genomes.0;
    genome2 = new_genomes.1;
    g1.set_agent_genome_at(rn,genome1);
    g2.set_agent_genome_at(rn,genome2);
    (g1,g2)
}

pub fn single_site_cross(vec1: Vec<i32>, vec2: Vec<i32>) -> (Vec<i32>,Vec<i32>) {
    assert_eq!(vec1.len(),vec2.len());
    let mut new_v1 = Vec::new();
    let mut new_v2 = Vec::new();

    let rn_idx : usize = rand::thread_rng().gen_range(0,vec1.len());
    for idx in 0..vec1.len(){
        if idx<rn_idx{
            new_v1.push(vec1[idx]);
            new_v2.push(vec2[idx]);
        } else {
            new_v1.push(vec2[idx]);
            new_v2.push(vec1[idx]);
        }
    }

    (new_v1,new_v2)

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_fn_normalise() {
        let mut sw = vec![1.0,2.0,5.0,2.0];
        sw = normalise(sw);
        assert_eq!(sw,vec![0.1,0.2,0.5,0.2]);
    }

    #[test]
    fn prob_1_true(){
        for _ in 0..100 {
            assert!(true_with_prob(1.0));
        }
    }
    #[test]
    fn prob_0_false(){
        for _ in 0..100{
            assert!(!true_with_prob(0.0));
        }
    }
    #[test]
    fn prob_dist_1_test(){
        for _ in 0..100{
            assert_eq!(3,idx_from_prob_dist(&vec![0.001,0.0,0.0,0.999,0.0]));
        }
    }

}


