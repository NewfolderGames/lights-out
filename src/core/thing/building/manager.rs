use std::collections::HashMap;
use crate::core::modifier::{ModifierEntry, ModifierStorage};
use crate::core::thing::building::Building;

pub struct BuildingManager {
    
    buildings: HashMap<String, Building>,

    is_upkeep_dirty: bool,
    is_output_dirty: bool,
    is_modifiers_dirty: bool,
    calculated_upkeep: HashMap<String, f64>,
    calculated_output: HashMap<String, f64>,
    calculated_modifiers: HashMap<String, ModifierEntry>,

}

impl BuildingManager {
    
    pub fn new() -> Self {
        
        Self {
            buildings: HashMap::new(),
            is_upkeep_dirty: true,
            is_output_dirty: true,
            is_modifiers_dirty: true,
            calculated_modifiers: HashMap::new(),
            calculated_upkeep: HashMap::new(),
            calculated_output: HashMap::new(),
        }
        
    }
    
    pub fn calculate(&mut self, modifier_storage: &ModifierStorage) {
        
        if !self.is_upkeep_dirty && !self.is_output_dirty && !self.is_modifiers_dirty { return; }
        
        
        
    }
    
}

