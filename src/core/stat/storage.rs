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
        let mut values: HashMap<String, (f64, f64, f64)> = HashMap::new();

        self.modifiers
            .iter()
            .for_each(|entry| {

                let val = values.entry(entry.get_name().to_owned()).or_insert((0f64, 0f64, 1f64));
                
                match entry.get_calculation_method() {
                    ModifierCalculationMethod::Addition => val.0 += entry.get_value(),
                    ModifierCalculationMethod::Multiplicative => val.1 += entry.get_value(),
                    ModifierCalculationMethod::Multiply => val.2 += entry.get_value(),
                };

            });

        values.iter().for_each(|(k, v)| {

            let value = (v.0 + (1f64 + v.1)) * v.2;
            calculated.insert(k.to_owned(), value);
            
        });

        self.is_dirty = false;
        self.calculated = calculated;

    }

    pub fn is_dirty(&self) -> bool {

        self.is_dirty

    }
    
    pub fn get_calculated(&self) -> &HashMap<String, f64> {
        
        &self.calculated
        
    }

}
