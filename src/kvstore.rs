use std::collections::HashMap;
use std::env;
// use std::fs;
// use std::io;

#[allow(dead_code)]

pub struct Kv<'a> {
    pub name: &'a str,
    pub filepath: String,
    pub cache: HashMap<i32, String>
}

impl Kv<'_> {
    pub fn new(name: &str) -> Kv {
        Kv { 
            name: name,
            filepath:  env::current_dir().expect("Failed to get current working directory").display().to_string() + "/.data",
            cache: HashMap::new(), 
        }
    }

    pub fn printer(&mut self) {
        println!("{}, {}, {}", self.name, self.filepath, self.cache.len());
    }

    pub fn put(&mut self, key: i32, value: &str) -> Result<(), &str> {
        self.cache.insert(key, String::from(value));

       return Ok(());
    }

    pub fn get(&mut self, key: i32) -> Result<&String, &str> {
        return match self.cache.get(&key) {
            Some(value) => Ok(value),
            None => Err("Key not found"),
        };
    }

    pub fn remove(&mut self, key: i32)  -> Result<(), &str> {
        return match self.cache.remove(&key) {
            Some(_value) => Ok(()),
            None => Err("Key not found")
        };
        //add functionality to check bloom filter of pages and see if 
    }

}

