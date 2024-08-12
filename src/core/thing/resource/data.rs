use crate::core::modifier::ModifierStorage;
use crate::core::thing::resource::asset::ResourceAsset;

pub struct Resource {

    asset: ResourceAsset,

    count: f64,
    capacity: f64,
    production: f64,
    consumption: f64,

    calculated_modifiers: ModifierStorage,
    
    is_unlocked: bool,

}

impl From<ResourceAsset> for Resource {

    fn from(asset: ResourceAsset) -> Self {

        Resource {
            asset: asset,
            count: 0f64,
            capacity: 0f64,
            production: 0f64,
            consumption: 0f64,
            calculated_modifiers: ModifierStorage::new(),
            is_unlocked: false,
        }

    }

}

impl Resource {

    pub fn asset(&self) -> &ResourceAsset {

        &self.asset

    }
    
    pub fn calculate(&mut self) {
        
        unimplemented!();
        
    }
    
    pub fn count(&self) -> f64 {
        
        self.count
        
    }

    pub fn add_count(&mut self, count: f64) {

        if self.count >= self.capacity && count >= 0f64 { return; }
        else if self.count + count >= self.capacity {self.count = self.capacity; }
        else if self.count + count > 0f64 { self.count += count; }
        else { self.count = 0f64; }

    }

    pub fn set_count(&mut self, count: f64) {

        self.count = count;

    }

    pub fn set_capacity(&mut self, capacity: f64) {

        self.capacity = capacity;

    }

    pub fn is_unlocked(&self) -> bool {

        self.is_unlocked

    }

    pub fn production(&self) -> f64 {

        self.production

    }

    pub fn set_production(&mut self, production: f64) {

        self.production = production;

    }

    pub fn consumption(&self) -> f64 {

        self.consumption

    }

    pub fn set_consumption(&mut self, consumption: f64) {

        self.consumption = consumption;

    }

    pub fn produce(&mut self) {
        
        let value = self.production - self.consumption;
        
        if self.count >= self.capacity && value >= 0f64 { return }
        else if self.count + value >= self.capacity { self.count = self.capacity; }
        else if self.count + value > 0f64 { self.count += value; }
        else { self.count = 0f64;  }
        
    }

    pub fn is_drained(&self) -> bool {

        self.consumption > self.production && self.count == 0f64

    }
    
    pub fn unlock(&mut self, is_unlocked: bool) {

        self.is_unlocked = is_unlocked;

    }
    
    pub fn calculated_modifiers(&self) -> &ModifierStorage {
        
        &self.calculated_modifiers
        
    }

}
