use crate::core::thing::building::{Building, BuildingAsset};
use crate::core::thing::modifier::ModifierStorage;
use crate::core::thing::resource::{ResourceManager, ResourceStorage};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// Building manager.
pub struct BuildingManager {
    
    /// Buildings.
    buildings: HashMap<String, Building>,
    
    /// Calculated building upkeeps.
    calculated_upkeeps: ResourceStorage,
    /// Calculated building outputs.
    calculated_outputs: ResourceStorage,
    /// Calculated building modifiers.
    calculated_modifiers: ModifierStorage,
    /// Calculated building storages.
    calculated_storages: ResourceStorage,

}

impl BuildingManager {

    /// Creates a new building manager.
    pub fn new() -> Self {
        
        Self {
            buildings: HashMap::new(),
            calculated_upkeeps: ResourceStorage::new(),
            calculated_outputs: ResourceStorage::new(),
            calculated_modifiers: ModifierStorage::new(),
            calculated_storages: ResourceStorage::new(),
        }
        
    }

    /// Iterate through all buildings.
    pub fn iter(&self) -> Iter<String, Building> {

        self.buildings.iter()

    }

    /// Returns a single building.
    pub fn get(&self, name: &str) -> Option<&Building> {

        self.buildings.get(name)

    }

    /// Unlocks a building.
    pub fn unlock(&mut self, name: &str) {

        if let Some(v) = self.buildings.get_mut(name) { v.unlock() }

    }

    /// Unlocks a building production.
    pub fn unlock_production(&mut self, name: &str, production: &str) {

        if let Some(v) = self.buildings.get_mut(name) { v.unlock_production(production) }

    }

    /// Adds count to a building.
    pub fn add_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.add_count(count); }

    }

    /// Sets building's count.
    pub fn set_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.set_count(count); }

    }

    /// Adds active count to a building.
    pub fn add_active_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.add_active_count(count); }

    }

    /// Sets a building's active count.
    pub fn set_active_count(&mut self, name: &str, count: i32) {

        if let Some(v) = self.buildings.get_mut(name) { v.set_active_count(count); }

    }
    
}

/// Implementations related to loading and registering buildings.
impl BuildingManager {

    /// Loads building from string.
    ///
    /// # Params
    ///
    /// - `building_asset_str`: JSON string of building asset.
    pub fn load_from_str(&mut self, building_asset_str: &str) -> serde_json::Result<()> {

        let result = serde_json::from_str(building_asset_str)?;
        self.load_from_asset(result);
        Ok(())

    }

    /// Loads building from asset.
    pub fn load_from_asset(&mut self, building_asset: BuildingAsset) {

        let building = Building::from(building_asset);
        self.add(building);

    }

    /// Adds a new building.
    pub fn add(&mut self, building: Building) {

        self.buildings.insert(building.asset().name.clone(), building);

    }
    
}

/// Implementations related to calculation.
impl BuildingManager {

    /// Calculates all buildings.
    ///
    /// # Params
    ///
    /// - `modifier_storage`: Modifiers that can be used for calculating buildings.
    /// - `resource_manager`: Resource manager for checking whether resources are available or not.
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
    
    /// Returns calculated upkeeps.
    pub fn calculated_upkeeps(&self) -> &ResourceStorage {

        &self.calculated_upkeeps

    }

    /// Returns calculated outputs.
    pub fn calculated_outputs(&self) -> &ResourceStorage {

        &self.calculated_outputs

    }
    
    /// Returns calculated modifiers.
    pub fn calculated_modifiers(&self) -> &ModifierStorage {

        &self.calculated_modifiers

    }

    // Returns calculated storages.
    pub fn calculated_storages(&self) -> &ResourceStorage {

        &self.calculated_storages

    }
    
}
