use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResourceAsset {

    /// Name of the resource.
    pub name: String,
    /// Category of the resource.
    pub category: String,
    
    // Modifiers that resource passively generates.
    pub modifiers: Vec<ResoureModifiers>,
    
}

#[derive(Deserialize)]
pub struct ResoureModifiers {
    
    /// Name of the modifier
    pub name: String,
    /// Base value of the modifier.
    pub base: f64,
    /// Generated value of the modifier.
    pub value: f64,
    /// Calculation method of the modifier.
    pub calculation: String,
    
}