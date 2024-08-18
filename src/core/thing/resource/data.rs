use crate::core::modifier::{ModifierCalculationMethod, ModifierEntry, ModifierStorage};
use crate::core::thing::resource::asset::ResourceAsset;

/// Resource
pub struct Resource {

    /// Resource's asset.
    asset: ResourceAsset,

    /// Resource count.
    count: f64,
    /// Capacity of the resource.
    capacity: f64,
    /// Resource production per tick.
    production: f64,
    /// Resource consumption per tick.
    consumption: f64,

    /// Calculated modifiers generated from the resource.
    calculated_modifiers: ModifierStorage,

    /// Is the resource unlocked?
    is_unlocked: bool,

}

impl From<ResourceAsset> for Resource {

    /// Creates a resource object from an asset.
    fn from(asset: ResourceAsset) -> Self {

        Resource {
            count: 0f64,
            capacity: asset.base_capacity,
            production: 0f64,
            consumption: 0f64,
            calculated_modifiers: ModifierStorage::new(),
            is_unlocked: false,
            asset,
        }

    }

}

impl Resource {

    /// Returns the resource's asset.
    pub fn asset(&self) -> &ResourceAsset {

        &self.asset

    }

    /// Returns the resource's count.
    pub fn count(&self) -> f64 {
        
        self.count
        
    }

    /// Adds to the resource's count.
    pub fn add_count(&mut self, count: f64) {

        if self.count >= self.capacity && count >= 0f64 { return; }
        else if self.count + count >= self.capacity {self.count = self.capacity; }
        else if self.count + count > 0f64 { self.count += count; }
        else { self.count = 0f64; }

    }

    /// Sets the resource's count.
    pub fn set_count(&mut self, count: f64) {

        self.count = count;

    }
    
    /// Sets the resource's capacity.
    pub fn capacity(&self) -> f64 {
        
        self.capacity
        
    }

    /// Sets the resource's capacity.
    pub fn set_capacity(&mut self, capacity: f64) {

        self.capacity = (self.asset.base_capacity + capacity).max(self.asset.base_capacity);

    }

    /// Is the resource unlocked?
    pub fn is_unlocked(&self) -> bool {

        self.is_unlocked

    }

    /// Unlocks the resource.
    pub fn unlock(&mut self, is_unlocked: bool) {

        self.is_unlocked = is_unlocked;

    }

}

/// Implementations related to resource production.
impl Resource {

    /// Returns resource's production.
    pub fn production(&self) -> f64 {

        self.production

    }

    /// Sets resource's production.
    pub fn set_production(&mut self, production: f64) {

        self.production = production;

    }

    /// Returns resource's consumption.
    pub fn consumption(&self) -> f64 {

        self.consumption

    }

    /// Sets resource's consumption.
    pub fn set_consumption(&mut self, consumption: f64) {

        self.consumption = consumption;

    }

    /// Produces resource.
    pub fn produce(&mut self) {

        let value = self.production - self.consumption;

        if self.count >= self.capacity && value >= 0f64 { return }
        else if self.count + value >= self.capacity { self.count = self.capacity; }
        else if self.count + value > 0f64 { self.count += value; }
        else { self.count = 0f64;  }

    }

    /// Returns true if resource is 0 and consumption is bigger than production.
    pub fn is_drained(&self) -> bool {

        self.consumption > self.production && self.count == 0f64

    }

}

/// Implementations related to resource calculation.
impl Resource {

    /// Calculates resource's modifiers.
    pub fn calculate(&mut self) {

        self.calculated_modifiers.clear();

        self.asset.modifiers.iter().for_each(|m| {
            self.calculated_modifiers.add(ModifierEntry::new(m.name.clone(), m.value, ModifierCalculationMethod::from_str(m.calculation.as_str())));
        });

    }

    /// Calculated resource's modifiers.
    pub fn calculated_modifiers(&self) -> &ModifierStorage {

        &self.calculated_modifiers

    }

}
