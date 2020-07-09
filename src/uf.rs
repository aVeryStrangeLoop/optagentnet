use crate::config::Config;

pub fn utility_function(resvec: &Vec<f64>,cfg:&Config) -> f64 {
    resvec.iter().sum()
}
