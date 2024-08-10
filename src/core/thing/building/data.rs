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
    calculated_storage: HashMap<String, f64>,
    
    is_unlocked: bool,

}

impl From<BuildingAsset> for Building {

    fn from(asset: BuildingAsset) -> Self {

        Self {
            asset,
            count: 0,
            active_count: 0,
            active_productions: HashSet::new(),
            is_dirty: false,
            calculated_upkeep: HashMap::new(),
            calculated_output: HashMap::new(),
            calculated_modifiers: ModifierStorage::new(),
            calculated_storage: HashMap::new(),
            is_unlocked: false,
        }

    }

}

impl Building {

    pub fn asset(&self) -> &BuildingAsset {

        &self.asset

    }

    pub fn calculate(&mut self) {

        if !self.is_dirty { return; }
        
        let mut upkeep: HashMap<String, f64> = HashMap::new();
        let mut output: HashMap<String, f64> = HashMap::new();
        let mut modifiers = ModifierStorage::new();
        let mut storage: HashMap<String, f64> = HashMap::new();
        
        self.active_productions.iter().for_each(|production_name| {
            
            let production = self.asset
                .productions
                .iter()
                .find(|data| data.name == production_name.to_string());
            
            if let Some(entry) = production {

                entry.upkeeps
                    .iter()
                    .for_each(|v| {
                        upkeep.insert(v.name.to_string(), v.value * self.active_count as f64);
                    });

                entry.outputs
                    .iter()
                    .for_each(|v| {
                        output.insert(v.name.to_string(), v.value * self.active_count as f64);
                    });
                
                entry.modifiers
                    .iter()
                    .for_each(|v| {

                        let calculation_method = match v.name.as_str() {
                            "addition" => ModifierCalculationMethod::Addition,
                            "multiplicative" => ModifierCalculationMethod::Multiplicative,
                            "multiply" => ModifierCalculationMethod::Multiply,
                            _ => panic!("wronf modifier calculation method"),
                        };
                        
                        modifiers.add_modifier(ModifierEntry::new(v.name.clone(), v.value, self.active_count, calculation_method));
                        
                    });

                entry.storage
                    .iter()
                    .for_each(|v| {
                        storage.insert(v.name.to_string(), v.value * self.active_count as f64);
                    });
                
            }
            
        });
        
        self.calculated_upkeep = upkeep;
        self.calculated_output = output;
        self.calculated_modifiers = modifiers;
        self.calculated_storage = storage;
        
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

}
