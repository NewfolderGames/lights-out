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
    
    /// Resource upkeep of the building.
    pub upkeeps: Vec<BuildingUpkeep>,
    /// Resource output of the building.
    pub outputs: Vec<BuildingOutput>,
    /// Modifiers generated from the building.
    pub modifiers: Vec<BuildingModifier>,

}

#[derive(Deserialize)]
struct BuildingPrice {
    
    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,
    
}

#[derive(Deserialize)]
struct BuildingUpkeep {
    
    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,
    
}

#[derive(Deserialize)]
struct BuildingOutput {
    
    /// Resource name.
    pub name: String,
    /// Resource value.
    pub value: f64,
    
}

#[derive(Deserialize)]
struct BuildingModifier {
    
    /// Name of the modifier
    pub name: String,
    /// Base value of the modifier.
    pub base: f64,
    /// Generated value of the modifier.
    pub value: f64,
    /// Calculation method of the modifier.
    pub calculation: String,
    
}
