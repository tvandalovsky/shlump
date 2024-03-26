// This file will create a guassian mixture model 

pub struct GMM {
    pub data: Vec<f64>,
    pub num_clusters: usize,
    pub num_iterations: usize,

    // allow to use random initialzed data
    // pub means: Vec<f64>,
    // pub variances: Vec<f64>,
    // pub weights: Vec<f64>,
    // pub responsibilities: Vec<Vec<f64>>,
}