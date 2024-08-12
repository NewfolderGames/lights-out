use std::collections::HashMap;
use crate::core::modifier::ModifierStorage;
use crate::core::thing::resource::{Resource, ResourceStorage};

pub struct ResourceManager {
    
    resources: HashMap<String, Resource>,
    
    calculated_modifiers: ModifierStorage,
    
}

impl ResourceManager{
    
    pub fn new() -> Self {
        
        Self {
            resources: HashMap::new(),
            calculated_modifiers: ModifierStorage::new(),
        }
        
    }
    
    pub fn calculate(&mut self) {
        
        self.calculated_modifiers.clear();
        self.resources
            .iter_mut()
            .for_each(|(_, r)| {

                r.produce();
                r.calculate();
                self.calculated_modifiers.combine(r.calculated_modifiers())
                
            });
        
    }
    
    pub fn set_production(&mut self, resource_storage: &ResourceStorage) {
        
        unimplemented!();
        
    }
    
    pub fn set_consumption(&mut self, resource_storage: &ResourceStorage) {
        
        unimplemented!();
        
    }
    
    pub fn calculated_modifiers(&self) -> &ModifierStorage {
        
        &self.calculated_modifiers
        
    }
    
    pub fn is_drained(&self, name: &str) -> bool {
        
        self.resources
            .get(name)
            .map(|r| r.is_drained())
            .unwrap_or(true)
        
    }
    
    pub fn count(&self, name: &str) -> f64 {
        
        self.resources
            .get(name)
            .map(|r| r.count())
            .unwrap_or(0f64)
        
    }
    
}