use super::BuildingAsset;
use crate::core::modifier::{ModifierCalculationMethod, ModifierEntry, ModifierStorage};
use crate::core::thing::resource::ResourceStorage;
use std::collections::HashSet;

pub struct Building {

    asset: BuildingAsset,

    count: i32,
    active_count: i32,
    active_productions: HashSet<String>,

    is_dirty: bool,
    calculated_upkeeps: ResourceStorage,
    calculated_outputs: ResourceStorage,
    calculated_modifiers: ModifierStorage,
    calculated_storages: ResourceStorage,
    calculated_prices: ResourceStorage,

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
            calculated_upkeeps: ResourceStorage::new(),
            calculated_outputs: ResourceStorage::new(),
            calculated_modifiers: ModifierStorage::new(),
            calculated_storages: ResourceStorage::new(),
            calculated_prices: ResourceStorage::new(),
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

        self.calculated_upkeeps.clear();
        self.calculated_outputs.clear();
        self.calculated_modifiers.clear();
        self.calculated_storages.clear();
        self.calculated_prices.clear();

        self.active_productions.iter().for_each(|production_name| {

            let production = self.asset
                .productions
                .iter()
                .find(|data| data.name == *production_name);

            if let Some(entry) = production {

                entry.upkeeps
                    .iter()
                    .for_each(|v| {
                        self.calculated_upkeeps.add(v.name.to_string(), get_modified_value("upkeep", v.value, self.active_count, &self.asset, modifier_storage));
                    });

                entry.outputs
                    .iter()
                    .for_each(|v| {
                        self.calculated_outputs.add(v.name.to_string(), get_modified_value("output", v.value, self.active_count, &self.asset, modifier_storage));
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

                        self.calculated_modifiers.add(ModifierEntry::new(v.name.clone(), v.value, calculation_method));

                    });

                entry.storages
                    .iter()
                    .for_each(|v| {
                        self.calculated_storages.add(v.name.to_string(), get_modified_value("storage", v.value, self.active_count, &self.asset, modifier_storage));
                    });

            }

        });

        self.asset.prices.iter().for_each(|price| {

            self.calculated_prices.add(price.name.to_string(), price.value * self.asset.price_multiplier.powi(self.count));

        });

        self.is_dirty = false;

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

    pub fn calculated_prices(&self) -> &ResourceStorage {

        &self.calculated_prices

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
