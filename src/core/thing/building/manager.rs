use crate::core::modifier::ModifierStorage;
use crate::core::thing::building::{Building, BuildingAsset};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use crate::core::thing::resource::{ResourceManager, ResourceStorage};

pub struct BuildingManager {
    buildings: HashMap<String, Building>,
    calculated_upkeeps: ResourceStorage,
    calculated_outputs: ResourceStorage,
    calculated_modifiers: ModifierStorage,
    calculated_storages: ResourceStorage,

}

impl BuildingManager {
    
    pub fn new() -> Self {
        
        Self {
            buildings: HashMap::new(),
            calculated_upkeeps: ResourceStorage::new(),
            calculated_outputs: ResourceStorage::new(),
            calculated_modifiers: ModifierStorage::new(),
            calculated_storages: ResourceStorage::new(),
        }
        
    }
    
    pub fn calculate(&mut self, modifier_storage: &ModifierStorage, resource_manager: &ResourceManager) {

        self.calculated_upkeeps.clear();
        self.calculated_outputs.clear();
        self.calculated_modifiers.clear();
        self.calculated_storages.clear();

        for (_, building) in self.buildings.iter_mut() {
            
            building.calculate(modifier_storage);
            let is_drained = building.calculated_upkeeps()
                .iter()
                .any(|(name, _)| {
                    resource_manager.is_drained(name)
                });

            if is_drained {

                self.calculated_upkeeps.combine(building.calculated_upkeeps());

            } else {

                self.calculated_upkeeps.combine(building.calculated_upkeeps());
                self.calculated_outputs.combine(building.calculated_outputs());
                self.calculated_modifiers.combine(building.calculated_modifiers());
                self.calculated_storages.combine(building.calculated_storages());

            }

        }
        
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

        if let Some(v) = self.buildings.get_mut(name) { v.add_count(count); }

    }

    pub fn set_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.set_count(count); }

    }

    pub fn add_active_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.add_active_count(count); }

    }

    pub fn set_active_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.set_active_count(count); }

    }

    pub fn calculated_upkeeps(&self) -> &ResourceStorage {
        
        &self.calculated_upkeeps
        
    }
    
    pub fn calculated_outputs(&self) -> &ResourceStorage {
        
        &self.calculated_outputs
        
    }
    
    pub fn calculated_modifiers(&self) -> &ModifierStorage {
        
        &self.calculated_modifiers
        
    }
    
    pub fn calculated_storages(&self) -> &ResourceStorage {
        
        &self.calculated_storages
        
    }
    
}
