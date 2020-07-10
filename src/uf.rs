use crate::config::Config;

pub fn utility_function(resvec: &Vec<f64>,cfg:&Config) -> f64 {
    match cfg.uf_number {
        0 => uf0(resvec,cfg),
        1 => uf1(resvec,cfg),
        2 => uf2(resvec,cfg),
        3 => uf3(resvec,cfg),
        4 => uf4(resvec,cfg),
        5 => uf5(resvec,cfg),
        _ => 0.0,
    }
}

/// UF 0 : Naive, sum of resources
fn uf0(resvec: &Vec<f64>,_cfg:&Config) -> f64 {
    resvec.iter().sum()
}

/// UF 1 : Constant compounding reward
fn uf1(resvec: &Vec<f64>,cfg:&Config) -> f64 {
    let mut sum = 0.0;
    for idx in 0..resvec.len(){
        sum += resvec[idx];
        for idx2 in 0..idx{
            sum += cfg.uf_param1 * s(resvec[idx]) * s(resvec[idx2]); 
        }
    }
    sum
}

/// UF 2 : Proportional compounding reward
fn uf2(resvec: &Vec<f64>,cfg:&Config) -> f64 {
    let mut sum = 0.0;
    for idx in 0..resvec.len(){
        sum += resvec[idx];
        for idx2 in 0..idx{
            sum += cfg.uf_param1 * s(resvec[idx]) * resvec[idx2]; 
        }
    }
    sum
}

/// UF 3 : Diversity, Number of unique tasks
fn uf3(resvec: &Vec<f64>,_cfg:&Config) -> f64 {
    resvec.iter().filter(|&n| *n>0.0).count() as f64
}


/// UF 4 : Naive x Diversity, Number of unique tasks * resvec sum
fn uf4(resvec: &Vec<f64>,_cfg:&Config) -> f64 {
    let sum : f64 = resvec.iter().sum(); 
    let div = resvec.iter().filter(|&n| *n>0.0).count() as f64;
    sum * div
}

/// UF 5 : Entropy
fn uf5(resvec: &Vec<f64>,_cfg:&Config) -> f64 {
    let normalised = normalise(resvec);
    let mut entropy = 0.0;
    for idx in 0..normalised.len(){
        if normalised[idx]>0.0{
            entropy -= normalised[idx] * normalised[idx].ln();
        }
    }
    entropy
}

fn s(num: f64) -> f64 {
    if num > 0.0 {1.0} else {0.0}
}

fn normalise(vec: &Vec<f64>) -> Vec<f64> {
    let sum: f64 = vec.iter().sum();
    vec.iter().map(|x| x/sum).collect()
}
