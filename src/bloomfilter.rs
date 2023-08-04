//use crate::kvstore::Kv;
use bit_vec::BitVec;
use fxhash::FxHasher;
use rand::seq::SliceRandom;
use std::hash::Hasher;

const TWO: f64 = 2.0;
const FALSE_POSITIVE_PROB: f64 = 0.000001;

pub struct Bf {
    pub name: String,
    pub bit_vec: BitVec,
    pub num_elements: i32,
    value_log: Vec<String>,
    num_hash: f64,
    hash_functions: Vec<Box<dyn Fn(&str) -> u64>>,
}

impl Bf {
    pub fn new(name: &str) -> Bf {
        Bf {
            name: String::from(name),
            bit_vec: BitVec::from_elem(1000, false),
            value_log: vec![],
            //hash_seeds: vec![],
            num_hash: -(FALSE_POSITIVE_PROB.log2()).round(),
            hash_functions: vec![],
            num_elements: 0,
        }
    }
    #[allow(dead_code)]
    fn setup(&mut self) {
        self.hash_fn_generator();
    }

    // num_bits: (((-1.44)*FALSE_POSITIVE_PROB.log2())*10.0) as i32
    pub fn printer(&mut self) {
        println!(
            "{}: num_hash {}, num_elements {}, bit_vec len: {}",
            self.name,
            self.num_hash,
            self.num_elements,
            self.bit_vec.len()
        );
        println!(
            "optimal bit num: {}",
            -1.0 * (self.num_elements as f64 * FALSE_POSITIVE_PROB.ln()) / f64::powf(TWO.ln(), 2.0)
        );
    }

    pub fn set(&mut self, bf_value: &str) -> bool {
        if self.num_elements == 0 {
            self.hash_fn_generator();
        }

        self.num_elements += 1;

        self.value_log.push(bf_value.to_string());

        let optimal_bf_len =
            -1.0 * (self.num_elements as f64 * FALSE_POSITIVE_PROB.ln()) / f64::powf(TWO.ln(), 2.0);
        //println!("optimal bf len: {}, bit_vec len: {}", optimal_bf_len, self.bit_vec.len());

        if optimal_bf_len > self.bit_vec.len() as f64 {
            self.do_bloom_refactor();
        }

        //let mut i = 0;
        for hash_function in self.hash_functions.iter() {
            let index = (hash_function(bf_value) % self.bit_vec.len() as u64) as usize;
            //println!("index {}: {:?}", i, index);
            self.bit_vec.set(index, true);
            //println!("BitVec values set to true: {:?}", self.bit_vec.get(index));
            //i += 1;
        }

        return true;
    }

    pub fn look_up(&mut self, bf_value: &str) -> bool {
        //let mut i = 0;
        for hash_function in self.hash_functions.iter() {
            let index = (hash_function(bf_value) % self.bit_vec.len() as u64) as usize;
            //println!("index {}: {:?}", i, index);
            let checker = self.bit_vec.get(index).unwrap();
            if !checker {
                return false;
            }
            //i += 1;
        }
        return true;
    }

    fn hash_fn_generator(&mut self) {
        //-> Vec<Box<dyn Fn(&str) -> u64>>

        let mut rng = &mut rand::thread_rng();
        let range = 1..=self.bit_vec.len();
        let sample: Vec<usize> = range.collect();
        let seeds: Vec<usize> = sample
            .choose_multiple(&mut rng, self.num_hash as usize)
            .cloned()
            .collect();
        // let mut new_seeds = vec![];
        // new_seeds.clone_from(&seeds);
        // self.hash_seeds = new_seeds;
        // println!("{:?}", self.hash_seeds);

        let hash_functions: Vec<Box<dyn Fn(&str) -> u64>> = seeds
            .iter()
            .map(|seed| {
                let seed_clone = *seed;
                Box::new(move |data: &str| {
                    let mut hasher = FxHasher::default();
                    hasher.write(data.as_bytes());
                    hasher.write_usize(seed_clone);
                    hasher.finish()
                }) as Box<dyn Fn(&str) -> u64>
            })
            .collect();

        self.hash_functions = hash_functions;
        //return hash_functions;
    }

    fn do_bloom_refactor(&mut self) {
        //create new hash functions and seeds
        self.hash_fn_generator();
        let optimal_bf_len =
            -1.0 * (self.num_elements as f64 * FALSE_POSITIVE_PROB.ln()) / f64::powf(TWO.ln(), 2.0);
        self.bit_vec = BitVec::from_elem(optimal_bf_len.round() as usize * 5, false);

        for value in self.value_log.iter() {
            for hash_function in self.hash_functions.iter() {
                let index = (hash_function(value) % self.bit_vec.len() as u64) as usize;
                self.bit_vec.set(index, true);
            }
        }
        println!("number of values in log: {}", self.value_log.len());
        self.printer();
    }
}
