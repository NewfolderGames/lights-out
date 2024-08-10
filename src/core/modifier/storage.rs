use std::collections::HashMap;

use super::{ModifierCalculationMethod, ModifierEntry};

pub struct ModifierStorage {
    modifiers: HashMap<String, ModifierEntry>,
    is_dirty: bool,
    calculated: HashMap<String, f64>,
}

impl ModifierStorage {

    pub fn new() -> Self {

        Self {
            modifiers: HashMap::new(),
            is_dirty: false,
            calculated: HashMap::new(),
        }

    }

    pub fn calculate(&mut self) {

        let mut calculated: HashMap<String, f64> = HashMap::new();
        let mut values: HashMap<String, (f64, f64, f64, i32)> = HashMap::new();

        self.modifiers
            .iter()
            .for_each(|(_, entry)| {

                let val = values.entry(entry.name().to_owned()).or_insert((1f64, 1f64, 1f64, entry.count()));

                match entry.calculation_method() {
                    ModifierCalculationMethod::Addition => val.0 += entry.value(),
                    ModifierCalculationMethod::Multiplicative => val.1 += entry.value(),
                    ModifierCalculationMethod::Multiply => val.2 += entry.value(),
                };

            });

        values.iter().for_each(|(k, v)| {

            let value = v.0 * v.1 * v.2 * v.3 as f64;
            calculated.insert(k.to_owned(), value);

        });

        self.is_dirty = false;
        self.calculated = calculated;

    }
    
    pub fn clear(&mut self) {
        
        self.is_dirty = false;
        self.modifiers.clear();
        self.calculated.clear();
        
    }
    
    pub fn add_modifier(&mut self, modifier_entry: ModifierEntry) {
        
        if let Some(entry) = self.modifiers.get_mut(modifier_entry.key().as_str()) {
            
            entry.add_count(modifier_entry.count());
            entry.add_value(modifier_entry.value());
            
        } else {

            self.modifiers.insert(modifier_entry.key(), modifier_entry);
            
        }
        
    }

    pub fn is_dirty(&self) -> bool {

        self.is_dirty

    }

    pub fn calculated(&self) -> &HashMap<String, f64> {

        &self.calculated

    }

    pub fn set_count(&mut self, key: &str, value: i32) {

        if let Some(v) = self.modifiers.get_mut(key) {

            v.set_count(value);
            self.is_dirty = true;

        }

    }

    pub fn add_count(&mut self, key: &str, value: i32) {

        if let Some(v) = self.modifiers.get_mut(key) {

            v.add_count(value);
            self.is_dirty = true;

        }

    }

}
