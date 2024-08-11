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
                        upkeeps.insert(v.name.to_string(), get_modified_value("upkeep", v.value, self.active_count, &self.asset, modifier_storage));
                    });

                entry.outputs
                    .iter()
                    .for_each(|v| {
                        outputs.insert(v.name.to_string(), get_modified_value("output", v.value, self.active_count, &self.asset, modifier_storage));
                    });

                entry.modifiers
                    .iter()
                    .for_each(|v| {

                        let calculation_method = match v.name.as_str() {
                            "base" => ModifierCalculationMethod::Base,
                            "addition" => ModifierCalculationMethod::Addition,
                            "multiplicative" => ModifierCalculationMethod::Multiplicative,
                            _ => panic!("wrong modifier calculation method"),
                        };

                        modifiers.add_modifier(ModifierEntry::new(v.name.clone(), v.value, calculation_method));

                    });

                entry.storages
                    .iter()
                    .for_each(|v| {
                        storages.insert(v.name.to_string(), get_modified_value("storage", v.value, self.active_count, &self.asset, modifier_storage));
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

fn get_modified_value(key: &str, original_value: f64, active_count: i32, asset: &BuildingAsset, modifier_storage: &ModifierStorage) -> f64 {

    let mut value = original_value;
    value +=
        modifier_storage.value(&format!("building.name.{}.{}", asset.name, key), ModifierCalculationMethod::Base) +
        modifier_storage.value(&format!("building.category.{}.{}", &asset.category, key), ModifierCalculationMethod::Base) +
        modifier_storage.value(&format!("building.global.{}", key), ModifierCalculationMethod::Base);
    value *= 1f64 +
        modifier_storage.value(&format!("building.name.{}.{}", asset.name, key), ModifierCalculationMethod::Multiplicative) +
        modifier_storage.value(&format!("building.category.{}.{}", asset.category, key), ModifierCalculationMethod::Multiplicative) +
        modifier_storage.value(&format!("building.global.{}", key), ModifierCalculationMethod::Multiplicative);
    value +=
        modifier_storage.value(&format!("building.name.{}.{}", asset.name, key), ModifierCalculationMethod::Addition) +
        modifier_storage.value(&format!("building.category.{}.{}", asset.category, key), ModifierCalculationMethod::Addition) +
        modifier_storage.value(&format!("building.global.{}", key), ModifierCalculationMethod::Addition);
    value *= 1f64 + modifier_storage.value("global.speed", ModifierCalculationMethod::Multiplicative);
    value *= active_count as f64;
    
    value
    
}
