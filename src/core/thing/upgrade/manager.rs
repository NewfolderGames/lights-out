use super::{Upgrade, UpgradeAsset};
use crate::core::thing::modifier::ModifierStorage;
use std::collections::HashMap;

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

    /// Loads upgrade from string.
    ///
    /// # Params
    ///
    /// - `building_asset_str`: JSON string of building asset.
    pub fn load_from_str(&mut self, upgrade_asset_str: &str) -> serde_json::Result<()> {

        let result = serde_json::from_str(upgrade_asset_str)?;
        self.load_from_asset(result);
        Ok(())

    }

    /// Loads building from asset.
    pub fn load_from_asset(&mut self, upgrade_asset: UpgradeAsset) {

        let upgrade = Upgrade::from(upgrade_asset);
        self.add(upgrade);

    }

    /// Adds a new building.
    pub fn add(&mut self, upgrade: Upgrade) {

        self.upgrades.insert(upgrade.asset().name.clone(), upgrade);

    }

}

impl UpgradeManager {

    /// Calculates upgrades.
    pub fn calculate(&mut self) {

        self.calculated_modifiers.clear();

        for (_, upgrade) in self.upgrades.iter_mut() {

            upgrade.calculate();
            self.calculated_modifiers.combine(upgrade.calculated_modifiers());

        }

    }

    /// Returns calculated modifiers of upgrades.
    pub fn calculated_modifiers(&self) -> &ModifierStorage {

        &self.calculated_modifiers

    }

}
