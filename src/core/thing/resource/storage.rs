use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct ResourceStorage {
    resources: HashMap<String, f64>,
}

impl ResourceStorage {
    
    pub fn new() -> Self {
        
        Self {
            resources: HashMap::new(),
        }
        
    }
    
    pub fn clear(&mut self) {
        
        self.resources.clear();
        
    }
    
    pub fn iter(&self) -> Iter<String, f64> {
        
        self.resources.iter()
        
    }
    
    pub fn add(&mut self, name: String, value: f64) {
        
        *self.resources.entry(name).or_insert(0f64) += value;
        
    }
    
    pub fn combine(&mut self, other: &ResourceStorage) {
        
        other.iter().for_each(|v| { self.add(v.0.clone(), *v.1) })
        
    }
    
}