use crate::core::thing::resource::asset::ResourceAsset;

pub struct Resource {

    asset: ResourceAsset,

    count: f64,
    capacity: f64,

    is_unlocked: bool,

}

impl From<ResourceAsset> for Resource {

    fn from(asset: ResourceAsset) -> Self {

        Resource {
            asset: asset,
            count: 0f64,
            capacity: 0f64,
            is_unlocked: false,
        }

    }

}

impl Resource {

    pub fn asset(&self) -> &ResourceAsset {

        &self.asset

    }

    pub fn add_count(&mut self, count: f64) {

        self.count += count;

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
    
    pub fn unlock(&mut self, is_unlocked: bool) {

        self.is_unlocked = is_unlocked;

    }

}
