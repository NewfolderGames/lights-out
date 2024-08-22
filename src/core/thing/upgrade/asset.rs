use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpgradeAsset {

    /// Name of the upgrade.
    pub name: String,
    /// Category of the upgrade.
    pub category: String,

    /// Price of the upgrade.
    pub price: Vec<UpgradePrice>,
    /// Multiplier of the upgrade price.
    pub price_multiplier: f64,
    
    /// Things that the upgrade unlocks.
    pub unlocks: Vec<UpgradeUnlock>,
    /// Modifiers that the upgrade provides.
    pub modifiers: Vec<UpgradeModifier>,
    
}

#[derive(Deserialize)]
pub struct UpgradePrice {

    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,

}

#[derive(Deserialize)]
pub struct UpgradeUnlock {

    /// Unlock thing type.
    pub thing: String,
    /// Unlock name.
    pub name: String,

}

#[derive(Deserialize)]
pub struct UpgradeModifier {

    /// Name of the modifier
    pub name: String,
    /// Generated value of the modifier.
    pub value: f64,
    /// Calculation method of the modifier.
    pub calculation: String,

}
