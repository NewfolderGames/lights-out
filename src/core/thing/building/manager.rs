use crate::core::modifier::ModifierStorage;
use crate::core::thing::building::{Building, BuildingAsset};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct BuildingManager {
    
    buildings: HashMap<String, Building>,

    is_dirty: bool,

}

impl BuildingManager {
    
    pub fn new() -> Self {
        
        Self {
            buildings: HashMap::new(),
            is_dirty: false,
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

    pub fn iter(&self) -> Iter<String, Building> {

        self.buildings.iter()

    }

    pub fn get(&self, name: &str) -> Option<&Building> {

        self.buildings.get(name)

    }

    pub fn unlock(&mut self, name: &str) {

        if let Some(v) = self.buildings.get_mut(name) { v.unlock() }

    }

    pub fn unlock_production(&mut self, name: &str, production: &str) {

        if let Some(v) = self.buildings.get_mut(name) { v.unlock_production(production) }

    }

    pub fn add_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) {

            v.add_count(count);
            self.is_dirty = true;

        }

    }

    pub fn set_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) {

            v.set_count(count);
            self.is_dirty = true;

        }

    }

    pub fn add_active_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) {

            v.add_active_count(count);
            self.is_dirty = true;

        }

    }

    pub fn set_active_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) {

            v.set_active_count(count);
            self.is_dirty = true

        }

    }
    
}
