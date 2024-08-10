use std::collections::HashMap;
use crate::core::modifier::{ModifierEntry, ModifierStorage};
use crate::core::thing::building::{Building, BuildingAsset};

pub struct BuildingManager {
    
    buildings: HashMap<String, Building>,

    is_dirty: bool,
    calculated_upkeep: HashMap<String, f64>,
    calculated_output: HashMap<String, f64>,
    calculated_modifiers: HashMap<String, ModifierEntry>,

}

impl BuildingManager {
    
    pub fn new() -> Self {
        
        Self {
            buildings: HashMap::new(),
            is_dirty: false,
            calculated_modifiers: HashMap::new(),
            calculated_upkeep: HashMap::new(),
            calculated_output: HashMap::new(),
        }
        
    }
    
    pub fn calculate(&mut self, modifier_storage: &ModifierStorage) {
        
        if !self.is_dirty { return; }
        
        self.is_dirty = false;
        
    }

    pub fn load_from_str(&mut self, str: &str) -> Result<(), String> {

        let result: serde_json::Result<BuildingAsset> = serde_json::from_str(str);
        if let Ok(asset) = result {

            self.add_from_asset(asset);
            Ok(())

        } else {

            Err("Failed to parse building asset from string".to_string())

        }

    }

    pub fn add_from_asset(&mut self, building_asset: BuildingAsset) {

        let building = Building::from(building_asset);
        self.add(building);

    }

    pub fn add(&mut self, building: Building) {

        self.buildings.insert(building.asset().name.clone(), building);

    }
    
}

