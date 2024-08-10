use serde::Deserialize;

#[derive(Deserialize)]
pub struct BuildingAsset {

    /// Name of the building.
    pub name: String,
    /// Category of the building.
    pub category: String,

    /// Resource price of the building.
    pub prices: Vec<BuildingPrice>,
    /// Size of the building.
    pub size: f64,

    /// Productions of the buildings.
    pub productions: Vec<BuildingProduction>,

}

#[derive(Deserialize)]
pub struct BuildingPrice {

    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,

}

#[derive(Deserialize)]
pub struct BuildingProduction {

    /// Production name.
    pub name: String,
    /// Resource upkeep of the building production.
    pub upkeeps: Vec<BuildingUpkeep>,
    /// Resource output of the building production.
    pub outputs: Vec<BuildingOutput>,
    /// Modifiers generated from the building production.
    pub modifiers: Vec<BuildingModifier>,
    /// Storage size generated from the building production.
    pub storages: Vec<BuildingStorage>,

}

#[derive(Deserialize)]
pub struct BuildingUpkeep {

    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,

}

#[derive(Deserialize)]
pub struct BuildingOutput {

    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,

}

#[derive(Deserialize)]
pub struct BuildingStorage {

    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,

}

#[derive(Deserialize)]
pub struct BuildingModifier {

    /// Name of the modifier
    pub name: String,
    /// Generated value of the modifier.
    pub value: f64,
    /// Calculation method of the modifier.
    pub calculation: String,

}
