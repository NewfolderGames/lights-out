use std::collections::{HashMap, HashSet};
use crate::core::modifier::ModifierStorage;
use super::BuildingAsset;

pub struct Building {

    asset: BuildingAsset,

    count: i32,
    active_count: i32,
    active_productions: HashSet<String>,

    is_dirty: bool,
    modifier_storage: ModifierStorage,
    calculated_upkeep: HashMap<String, f64>,
    calculated_output: HashMap<String, f64>,

}

impl From<BuildingAsset> for Building {

    fn from(asset: BuildingAsset) -> Self {

        Self {
            asset,
            count: 0,
            active_count: 0,
            active_productions: HashSet::new(),
            is_dirty: false,
            modifier_storage: ModifierStorage::new(),
            calculated_upkeep: HashMap::new(),
            calculated_output: HashMap::new(),
        }

    }

}

impl Building {

    pub fn asset(&self) -> &BuildingAsset {

        &self.asset

    }

    pub fn calculate(&mut self) {

        let mut upkeep: HashMap<String, f64> = HashMap::new();
        let mut output: HashMap<String, f64> = HashMap::new();
        
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
                
            }
            
        });
        
        self.calculated_upkeep = upkeep;
        self.calculated_output = output;
        self.modifier_storage.calculate();
        
        self.is_dirty = false;

    }

}
