use crate::core::modifier::ModifierStorage;
use crate::core::thing::resource::ResourceStorage;
use crate::core::thing::upgrade::asset::UpgradeAsset;

/// Upgrade
pub struct Upgrade {

    /// Upgrade's asset.
    asset: UpgradeAsset,

    /// Upgrade count.
    count: i32,

    /// Calculated upgrade's modifiers.
    calculated_modifiers: ModifierStorage,
    /// Calculated building's price.
    calculated_prices: ResourceStorage,

    /// Is the upgrade unlocked?
    is_unlocked: bool,

}

impl From<UpgradeAsset> for Upgrade {

    fn from(asset: UpgradeAsset) -> Self {

        Self {
            asset,
            count: 0,
            calculated_modifiers: ModifierStorage::new(),
            calculated_prices: ResourceStorage::new(),
            is_unlocked: false,
        }

    }

}

impl Upgrade {

    /// Returns the upgrade's asset.
    pub fn asset(&self) -> &UpgradeAsset {

        &self.asset

    }

    // Is the upgrade unlocked?
    pub fn is_unlocked(&self) -> bool {

        self.is_unlocked

    }

    /// Unlocks the upgrade.
    pub fn unlock(&mut self) {

        self.is_unlocked = true;

    }
    
}