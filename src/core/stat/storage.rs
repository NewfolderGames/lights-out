use std::collections::{HashMap, HashSet};
use super::{ModifierEntry, ModifierCalculationMethod};

pub struct ModifierStorage {
    modifiers: HashSet<ModifierEntry>,
    is_dirty: bool,
    calculated: HashMap<String, f64>,
}

impl ModifierStorage {
    
    pub fn new() -> Self {
        
        Self {
            modifiers: HashSet::new(),
            is_dirty: false,
            calculated: HashMap::new(),
        }
        
    }
    
    pub fn calculate(&mut self) {
        
        let mut calculated: HashMap<String, f64> = HashMap::new();
        let mut flat: HashMap<String, f64> = HashMap::new();
        let mut multiplicative: HashMap<String, f64> = HashMap::new();
        let mut multiply: HashMap<String, f64> = HashMap::new();
        
        self.modifiers
            .iter()
            .for_each(|entry| {
                
               let map = match entry.get_calculation_method() {
                   ModifierCalculationMethod::Addition => &mut flat,
                   ModifierCalculationMethod::Multiplicative => &mut multiplicative,
                   ModifierCalculationMethod::Multiply => &mut multiply,
               };

                let val = map.entry(entry.get_name().to_owned()).or_insert(0f64);
                *val += entry.get_value();
                
                self.calculated.insert(entry.get_name().to_owned(), 0f64);
                
            });
        
        self.calculated.keys().for_each(|key| {
            
            let flatValue = *flat.get(key).unwrap_or(&1f64);
            let multiplicativeValue = *multiplicative.get(key).unwrap_or(&0f64);
            let multiplyValue = *multiply.get(key).unwrap_or(&1f64);
            
            
            
        });
        
        self.is_dirty = false;
        
    }
    
    pub fn is_dirty(&self) -> bool {
        
        self.is_dirty
        
    }
    
}
