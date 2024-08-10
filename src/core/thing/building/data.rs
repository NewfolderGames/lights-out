use std::collections::{HashMap, HashSet};
use crate::core::modifier::{ModifierCalculationMethod, ModifierEntry, ModifierStorage};
use super::BuildingAsset;

pub struct Building {

    asset: BuildingAsset,

    count: i32,
    active_count: i32,
    active_productions: HashSet<String>,

    is_dirty: bool,
    calculated_upkeep: HashMap<String, f64>,
    calculated_output: HashMap<String, f64>,
    calculated_modifiers: ModifierStorage,
    calculated_storages: HashMap<String, f64>,
    
    is_unlocked: bool,
    unlocked_productions: HashSet<String>,

}

impl From<BuildingAsset> for Building {

    fn from(asset: BuildingAsset) -> Self {

        let mut active_productions = HashSet::new();
        active_productions.insert("default".to_string());

        let mut unlocked_productions = HashSet::new();
        unlocked_productions.insert("default".to_string());
        
        Self {
            asset,
            count: 0,
            active_count: 0,
            active_productions: HashSet::new(),
            is_dirty: false,
            calculated_upkeep: HashMap::new(),
            calculated_output: HashMap::new(),
            calculated_modifiers: ModifierStorage::new(),
            calculated_storages: HashMap::new(),
            is_unlocked: false,
            unlocked_productions: unlocked_productions,
        }

    }

}

impl Building {

    pub fn asset(&self) -> &BuildingAsset {

        &self.asset

    }

    pub fn calculate(&mut self, modifier_storage: &ModifierStorage) {

        if !self.is_dirty { return; }
        
        let mut upkeeps: HashMap<String, f64> = HashMap::new();
        let mut outputs: HashMap<String, f64> = HashMap::new();
        let mut modifiers = ModifierStorage::new();
        let mut storages: HashMap<String, f64> = HashMap::new();
        
        self.active_productions.iter().for_each(|production_name| {
            
            let production = self.asset
                .productions
                .iter()
                .find(|data| data.name == *production_name);
            
            if let Some(entry) = production {

                entry.upkeeps
                    .iter()
                    .for_each(|v| {

                        let mut value = v.value;
                        value += modifier_storage.value(&format!("building.{}.upkeep", &self.asset.name), ModifierCalculationMethod::Base);
                        value *= 1f64 + modifier_storage.value(&format!("building.{}.upkeep", &self.asset.name), ModifierCalculationMethod::Multiplicative);
                        value *= 1f64 * modifier_storage.value(&format!("building.{}.upkeep", &self.asset.name), ModifierCalculationMethod::Multiply);
                        value += modifier_storage.value(&format!("building.{}.upkeep", &self.asset.name), ModifierCalculationMethod::Addition);
                        value *= self.active_count as f64;

                        upkeeps.insert(v.name.to_string(), value);
                        
                    });

                entry.outputs
                    .iter()
                    .for_each(|v| {

                        let mut value = v.value;
                        value += modifier_storage.value(&format!("building.{}.output", &self.asset.name), ModifierCalculationMethod::Base);
                        value *= 1f64 + modifier_storage.value(&format!("building.{}.output", &self.asset.name), ModifierCalculationMethod::Multiplicative);
                        value *= 1f64 * modifier_storage.value(&format!("building.{}.output", &self.asset.name), ModifierCalculationMethod::Multiply);
                        value += modifier_storage.value(&format!("building.{}.output", &self.asset.name), ModifierCalculationMethod::Addition);
                        value *= self.active_count as f64;

                        outputs.insert(v.name.to_string(), value);
                        
                    });
                
                entry.modifiers
                    .iter()
                    .for_each(|v| {

                        let calculation_method = match v.name.as_str() {
                            "base" => ModifierCalculationMethod::Base,
                            "addition" => ModifierCalculationMethod::Addition,
                            "multiplicative" => ModifierCalculationMethod::Multiplicative,
                            "multiply" => ModifierCalculationMethod::Multiply,
                            _ => panic!("wrong modifier calculation method"),
                        };
                        
                        modifiers.add_modifier(ModifierEntry::new(v.name.clone(), v.value, calculation_method));
                        
                    });

                entry.storages
                    .iter()
                    .for_each(|v| {

                        let mut value = v.value;
                        value += modifier_storage.value(&format!("building.{}.storage", &self.asset.name), ModifierCalculationMethod::Base);
                        value *= 1f64 + modifier_storage.value(&format!("building.{}.storage", &self.asset.name), ModifierCalculationMethod::Multiplicative);
                        value *= 1f64 * modifier_storage.value(&format!("building.{}.storage", &self.asset.name), ModifierCalculationMethod::Multiply);
                        value += modifier_storage.value(&format!("building.{}.storage", &self.asset.name), ModifierCalculationMethod::Addition);
                        value *= self.active_count as f64;
                        
                        storages.insert(v.name.to_string(), value);
                        
                    });
                
            }
            
        });
        
        self.calculated_upkeep = upkeeps;
        self.calculated_output = outputs;
        self.calculated_modifiers = modifiers;
        self.calculated_storages = storages;
        
        self.is_dirty = false;

    }
    
    pub fn calculated_upkeep(&self) -> &HashMap<String, f64> {
        
        &self.calculated_upkeep
        
    }

    pub fn calculated_output(&self) -> &HashMap<String, f64> {

        &self.calculated_output

    }

    pub fn calculated_modifiers(&self) -> &ModifierStorage {

        &self.calculated_modifiers

    }

    pub fn is_dirty(&self) -> bool {

        self.is_dirty

    }

    pub fn is_unlocked(&self) -> bool {

        self.is_unlocked

    }

    pub fn unlock(&mut self) {

        self.is_unlocked = true;

    }

    pub fn unlock_production(&mut self, key: &str) {

        if self.asset.productions.iter().any(|v| v.name == *key) {

            self.unlocked_productions.insert(key.to_string());

        }

    }

    pub fn is_production_unlocked(&self, key: &str) -> bool {

        self.unlocked_productions.iter().any(|v| v == key)

    }

    pub fn count(&self) -> i32 {

        self.count

    }

    pub fn add_count(&mut self, count: i32) {

        self.count += count;
        self.is_dirty = true;

    }

    pub fn set_count(&mut self, count: i32) {

        self.count = count;
        self.is_dirty = true;

    }

    pub fn active_count(&self) -> i32 {

        self.active_count

    }

    pub fn add_active_count(&mut self, count: i32) {

        self.active_count += count;
        self.is_dirty = true;

    }

    pub fn set_active_count(&mut self, count: i32) {

        self.active_count = count;
        self.is_dirty = true;

    }

}
