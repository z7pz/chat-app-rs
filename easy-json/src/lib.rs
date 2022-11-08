use serde::de::DeserializeOwned;
use serde::{Serialize};
use std::fs::File;
use std::io::prelude::*;
pub struct Database {
    pub path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
    pub fn remove<T: Serialize + DeserializeOwned, F: FnMut(&T) -> bool>(&self, find: F) {
        let mut file = match File::open(self.path.clone() + ".json") {
            Ok(file) => file,
            Err(_) => write_into_file(self.path.clone(), String::from("[]")).unwrap(),
        };
        let mut content = String::from("");
        file.read_to_string(&mut content).unwrap();
        let mut array: Vec<T> = serde_json::from_str(&content).unwrap();
        let index = array.iter().position(find).unwrap();
        array.remove(index);
        let serialized = serde_json::to_string(&array).unwrap();
        write_into_file(self.path.clone(), serialized).unwrap();
    }
    pub fn to_vec<T: Serialize + DeserializeOwned>(&self) -> Vec<T> {
        let mut file = match File::open(self.path.clone() + ".json") {
            Ok(file) => file,
            Err(_) => write_into_file(self.path.clone(), String::from("[]")).unwrap(),
        };
        let mut content = String::from("");
        file.read_to_string(&mut content).unwrap();
        let deserialized: Vec<T> = serde_json::from_str(&content).unwrap();
        return deserialized;
    }
    pub fn add<T: Serialize + DeserializeOwned>(&self, data: T) {
        let mut file = match File::open(self.path.clone() + ".json") {
            Ok(file) => file,
            Err(_) => write_into_file(self.path.clone(), String::from("[]")).unwrap(),
        };
        let mut content = String::from("");
        file.read_to_string(&mut content).unwrap();
        let mut array: Vec<T> = serde_json::from_str(&content).unwrap();
        array.push(data);
        let serialized = serde_json::to_string(&array).unwrap();
        write_into_file(self.path.clone(), serialized).unwrap();
    }
}
pub fn write_into_file(name: String, data: String) -> std::io::Result<File> {
    let mut file = File::create(name + ".json")?;
    file.write_all(data.as_bytes())?;
    Ok(file)
}
