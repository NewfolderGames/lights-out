use std::collections::HashMap;

use super::{ModifierCalculationMethod, ModifierEntry};

pub struct ModifierStorage {
    modifiers: HashMap<String, ModifierEntry>,
}

impl ModifierStorage {

    pub fn new() -> Self {

        Self {
            modifiers: HashMap::new(),
        }

    }
    
    pub fn clear(&mut self) {
        
        self.modifiers.clear();
        
    }
    
    pub fn add_modifier(&mut self, modifier_entry: ModifierEntry) {
        
        if let Some(entry) = self.modifiers.get_mut(modifier_entry.key().as_str()) {
            
            entry.add_value(modifier_entry.value());
            
        } else {

            self.modifiers.insert(modifier_entry.key(), modifier_entry);
            
        }
        
    }
    
    pub fn value(&self, name: &str, method: ModifierCalculationMethod) -> f64 {
        
        self.modifiers
            .get(&format!("{}.{}", name, method.key()))
            .map(|v| v.value())
            .unwrap_or(0f64)
        
    }
 
}
