use std::collections::HashMap;
use crate::core::modifier::ModifierStorage;
use crate::core::thing::resource::{Resource, ResourceAsset, ResourceStorage};

/// Resource manager.
pub struct ResourceManager {
    
    /// Resources.
    resources: HashMap<String, Resource>,
    
    /// Calculated resource modifiers.
    calculated_modifiers: ModifierStorage,
    
}

impl ResourceManager{
    
    /// Creates a new resource manager.
    pub fn new() -> Self {
        
        Self {
            resources: HashMap::new(),
            calculated_modifiers: ModifierStorage::new(),
        }
        
    }
    
    /// Calculates resources.
    pub fn calculate(&mut self) {
        
        self.calculated_modifiers.clear();
        for (_, resource) in self.resources.iter_mut() {

            resource.produce();
            resource.calculate();
            self.calculated_modifiers.combine(resource.calculated_modifiers())
            
        }
        
    }
    
    /// Sets production of resources.
    pub fn set_production(&mut self, resource_storage: &ResourceStorage) {
        
        for (name, value) in resource_storage.iter() {

            if let Some(resource) = self.resources.get_mut(name) {
                
                resource.set_production(*value);
                
            }
            
        }
        
    }
    
    /// Sets consumption of resources.
    pub fn set_consumption(&mut self, resource_storage: &ResourceStorage) {
        
        for (name, value) in resource_storage.iter() {

            if let Some(resource) = self.resources.get_mut(name) {

                resource.set_consumption(*value);

            }

        }
        
    }
    
    /// Returns calculated modifiers of the resources.
    pub fn calculated_modifiers(&self) -> &ModifierStorage {
        
        &self.calculated_modifiers
        
    }
    
    /// Returns true if the resource is currently empty and consumption is bigger than production.
    pub fn is_drained(&self, name: &str) -> bool {
        
        self.resources
            .get(name)
            .map(|r| r.is_drained())
            .unwrap_or(true)
        
    }
    
    /// Returns count of the resource.
    pub fn count(&self, name: &str) -> f64 {
        
        self.resources
            .get(name)
            .map(|r| r.count())
            .unwrap_or(0f64)
        
    }
    
}

/// Implementations related to loading and registering resources.
impl ResourceManager {

    /// Loads resource from string.
    ///
    /// # Params
    ///
    /// - `resource_asset_str`: JSON string of building asset.
    pub fn load_from_str(&mut self, resource_asset_str: &str) -> serde_json::Result<()> {

        let result = serde_json::from_str(resource_asset_str)?;
        self.load_from_asset(result);
        Ok(())

    }

    /// Loads building from asset.
    pub fn load_from_asset(&mut self, building_asset: ResourceAsset) {

        let building = Resource::from(building_asset);
        self.add(building);

    }

    /// Adds a new building.
    pub fn add(&mut self, building: Resource) {

        self.resources.insert(building.asset().name.clone(), building);

    }

}
