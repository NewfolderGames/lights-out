use std::collections::HashMap;
use crate::core::modifier::ModifierStorage;

pub struct BuildingManager {
    
    is_dirty: bool,
    calculated_buildings: HashMap<String, CalculatedBuilding>,
    calculated_upkeep: HashMap<String, f64>,
    calculated_output: HashMap<String, f64>,

}

impl BuildingManager {
    
    pub fn tick(&mut self, modifier_storage: ModifierStorage) {
        
        
        
    }
    
}

struct CalculatedBuilding {
    calculated_upkeep: HashMap<String, f64>,
    calculated_output: HashMap<String, f64>,
}
