use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResourceAsset {

    /// Name of the resource.
    pub name: String,
    /// Category of the resource.
    pub category: String,

    /// Base capacity value.
    pub base_capacity: f64,
    /// Modifiers that resource passively generates.
    pub modifiers: Vec<ResourceModifiers>,

}

#[derive(Deserialize)]
pub struct ResourceModifiers {

    /// Name of the modifier
    pub name: String,
    /// Generated value of the modifier.
    pub value: f64,
    /// Calculation method of the modifier.
    pub calculation: String,

}
