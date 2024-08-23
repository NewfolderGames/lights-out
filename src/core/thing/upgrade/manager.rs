use std::collections::HashMap;
use crate::core::modifier::ModifierStorage;
use crate::core::thing::upgrade::data::Upgrade;

pub struct UpgradeManager {

    upgrades: HashMap<String, Upgrade>,

    calculated_modifiers: ModifierStorage,

}

impl UpgradeManager {

    pub fn new() -> Self {

        Self {
            upgrades: HashMap::new(),
            calculated_modifiers: ModifierStorage::new(),
        }

    }

}

impl UpgradeManager {

    /// Calculates upgrades.
    pub fn calculate(&mut self) {

        unimplemented!()

    }

    /// Returns calculated modifiers of upgrades.
    pub fn calculated_modifiers(&self) -> &ModifierStorage {

        &self.calculated_modifiers

    }

}
