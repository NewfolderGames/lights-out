use super::BuildingAsset;
use crate::core::modifier::{ModifierCalculationMethod, ModifierEntry, ModifierStorage};
use crate::core::thing::resource::ResourceStorage;
use std::collections::HashSet;

/// Building
pub struct Building {

    /// Building's asset.
    asset: BuildingAsset,

    /// Building count.
    count: i32,
    /// Active building count.
    active_count: i32,

    /// Calculated building's upkeep.
    calculated_upkeeps: ResourceStorage,
    /// Calculated building's output.
    calculated_outputs: ResourceStorage,
    /// Calculated building's modifiers.
    calculated_modifiers: ModifierStorage,
    /// Calculated building's storage.
    calculated_storages: ResourceStorage,
    /// Calculated building's price.
    calculated_prices: ResourceStorage,

    /// Active building productions.
    active_productions: HashSet<String>,
    /// Unlocked productions or the building.
    unlocked_productions: HashSet<String>,

    /// Is the building unlocked?
    is_unlocked: bool,

}

impl From<BuildingAsset> for Building {

    /// Creates a building object from an asset.
    fn from(asset: BuildingAsset) -> Self {

        let mut active_productions = HashSet::new();
        active_productions.insert("default".to_string());

        let mut unlocked_productions = HashSet::new();
        unlocked_productions.insert("default".to_string());

        Self {
            asset,
            count: 0,
            active_count: 0,
            calculated_upkeeps: ResourceStorage::new(),
            calculated_outputs: ResourceStorage::new(),
            calculated_modifiers: ModifierStorage::new(),
            calculated_storages: ResourceStorage::new(),
            calculated_prices: ResourceStorage::new(),
            active_productions,
            unlocked_productions,
            is_unlocked: false,
        }

    }

}

impl Building {

    /// Returns the building's asset.
    pub fn asset(&self) -> &BuildingAsset {

        &self.asset

    }

    // Is the building unlocked?
    pub fn is_unlocked(&self) -> bool {

        self.is_unlocked

    }

    /// Unlocks the building.
    pub fn unlock(&mut self) {

        self.is_unlocked = true;

    }

}

/// Implementations related to building's count.
impl Building {

    /// Returns count of the building.
    pub fn count(&self) -> i32 {

        self.count

    }

    /// Adds count to the building.
    pub fn add_count(&mut self, count: i32) {

        self.count += count;

    }

    /// Sets the building's count.
    pub fn set_count(&mut self, count: i32) {

        self.count = count;

    }

    /// Active building count.
    pub fn active_count(&self) -> i32 {

        self.active_count

    }

    /// Adds active building count.
    pub fn add_active_count(&mut self, count: i32) {

        self.active_count += count

    }

    /// Sets active building count.
    pub fn set_active_count(&mut self, count: i32) {

        self.active_count = count;

    }

}

/// Implementations related to building's production.
impl Building {

    /// Returns active productions.
    pub fn active_productions(&self) -> &HashSet<String> {
        
        &self.active_productions
        
    }
    
    /// Returns unlocked productions.
    pub fn unlocked_productions(&self) -> &HashSet<String> {

        &self.unlocked_productions

    }

    /// Unlock production of the building.
    pub fn unlock_production(&mut self, key: &str) {

        if self.asset.productions.iter().any(|v| v.name == *key) {

            self.unlocked_productions.insert(key.to_string());

        }

    }

    /// Is the production unlocked?
    pub fn is_production_unlocked(&self, key: &str) -> bool {

        self.unlocked_productions.iter().any(|v| v == key)

    }

    /// Is the production unlocked?
    pub fn is_production_active(&self, key: &str) -> bool {

        self.active_productions.iter().any(|v| v == key)

    }
    
    /// Sets active production.
    /// 
    /// The production must be unlocked to change its active state.
    /// The 'default' production cannot be changed.
    pub fn set_active_production(&mut self, key: &str, active: bool) {
        
        if key == "default" { return; }
        
        if self.is_production_unlocked(key) {
            
            if active { self.active_productions.insert(key.to_string()); }
            else { self.active_productions.remove(key); }
            
        }
        
    }

}

/// Implementation related to building's calculation.
impl Building {

    /// Calculates building's upkeep, output, modifiers, storage and price.
    pub fn calculate(&mut self, modifier_storage: &ModifierStorage) {

        self.calculated_upkeeps.clear();
        self.calculated_outputs.clear();
        self.calculated_modifiers.clear();
        self.calculated_storages.clear();
        self.calculated_prices.clear();

        for production_name in self.active_productions.iter() {

            let production = self.asset
                .productions
                .iter()
                .find(|data| data.name == *production_name);

            if let Some(entry) = production {

                for upkeep in entry.upkeeps.iter() {

                    self.calculated_upkeeps.add(upkeep.name.to_string(), Self::get_modifier_value("upkeep", upkeep.value, self.active_count, &self.asset, modifier_storage));
                    
                }

                for output in entry.outputs.iter() {

                    self.calculated_outputs.add(output.name.to_string(), Self::get_modifier_value("output", output.value, self.active_count, &self.asset, modifier_storage));

                }

                for modifier in entry.modifiers.iter() {

                    self.calculated_modifiers.add(ModifierEntry::new(modifier.name.clone(), modifier.value * self.active_count as f64, ModifierCalculationMethod::from_str(modifier.calculation.as_str())));

                }

                for storage in entry.storages.iter() {

                    self.calculated_storages.add(storage.name.to_string(), Self::get_modifier_value("storage", storage.value, self.active_count, &self.asset, modifier_storage));

                }

            }
            
        }
        
        for price in self.asset.prices.iter() {

            self.calculated_prices.add(price.name.to_string(), (price.value * self.asset.price_multiplier.powi(self.count)).floor());
            
        }

    }

    /// Calculated upkeep of the building.
    pub fn calculated_upkeeps(&self) -> &ResourceStorage {

        &self.calculated_upkeeps

    }

    /// Calculated output of the building.
    pub fn calculated_outputs(&self) -> &ResourceStorage {

        &self.calculated_outputs

    }

    /// Calculated modifiers of the building.
    pub fn calculated_modifiers(&self) -> &ModifierStorage {

        &self.calculated_modifiers

    }

    /// Calculated storage of the building.
    pub fn calculated_storages(&self) -> &ResourceStorage {

        &self.calculated_storages

    }

    /// Calculated price of the building.
    pub fn calculated_prices(&self) -> &ResourceStorage {

        &self.calculated_prices

    }

    /// Gets combined modifier value from modifier storage.
    fn get_modifier_value(key: &str, original_value: f64, active_count: i32, asset: &BuildingAsset, modifier_storage: &ModifierStorage) -> f64 {

        let mut value = original_value;
        value +=
            modifier_storage.value(&format!("building.name.{}.{}", asset.name, key), ModifierCalculationMethod::Base) +
                modifier_storage.value(&format!("building.category.{}.{}", &asset.category, key), ModifierCalculationMethod::Base) +
                modifier_storage.value(&format!("building.global.{}", key), ModifierCalculationMethod::Base);
        value *= 1f64 +
            modifier_storage.value(&format!("building.name.{}.{}", asset.name, key), ModifierCalculationMethod::Additive) +
            modifier_storage.value(&format!("building.category.{}.{}", asset.category, key), ModifierCalculationMethod::Additive) +
            modifier_storage.value(&format!("building.global.{}", key), ModifierCalculationMethod::Additive);
        value +=
            modifier_storage.value(&format!("building.name.{}.{}", asset.name, key), ModifierCalculationMethod::Flat) +
                modifier_storage.value(&format!("building.category.{}.{}", asset.category, key), ModifierCalculationMethod::Flat) +
                modifier_storage.value(&format!("building.global.{}", key), ModifierCalculationMethod::Flat);
        value *= 1f64 + modifier_storage.value("global.speed", ModifierCalculationMethod::Additive);
        value *= active_count as f64;

        value

    }
    
}
