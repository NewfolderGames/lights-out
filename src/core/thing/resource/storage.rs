use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// Simple key-value storage for storing resource values.
pub struct ResourceStorage {
    resources: HashMap<String, f64>,
}

impl ResourceStorage {
    
    /// Creates a new storage.
    pub fn new() -> Self {
        
        Self {
            resources: HashMap::new(),
        }
        
    }
    
    /// Clears the storage.
    pub fn clear(&mut self) {
        
        self.resources.clear();
        
    }
    
    /// Iterates through all entries.
    pub fn iter(&self) -> Iter<String, f64> {
        
        self.resources.iter()
        
    }
    
    /// Adds value to an entry.
    pub fn add(&mut self, name: String, value: f64) {
        
        *self.resources.entry(name).or_insert(0f64) += value;
        
    }
    
    /// Combines values from other resource storage.
    pub fn combine(&mut self, other: &ResourceStorage) {
        
        other.iter().for_each(|v| { self.add(v.0.clone(), *v.1) })
        
    }
    
}