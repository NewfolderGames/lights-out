use thiserror::Error;
use crate::core::modifier::ModifierStorage;
use crate::core::thing::building::BuildingManager;
use crate::core::thing::resource::ResourceManager;
use crate::core::thing::upgrade::UpgradeManager;

pub struct ThingManager {

    building_manager: BuildingManager,
    resource_manager: ResourceManager,
    upgrade_manager: UpgradeManager,
    
    prev_tick_modifiers: ModifierStorage,

    is_first_tick: bool,

}

impl Default for ThingManager {

    fn default() -> Self {

        Self::new()

    }

}

impl ThingManager {

    pub fn new() -> Self {

        Self {
            building_manager: BuildingManager::new(),
            resource_manager: ResourceManager::new(),
            upgrade_manager: UpgradeManager::new(),
            prev_tick_modifiers: ModifierStorage::new(),
            is_first_tick: true,
        }

    }
    
    pub fn tick(&mut self) {
        
        let mut current_tick_modifiers = ModifierStorage::new();

        if !self.is_first_tick {

            self.resource_manager.set_production(self.building_manager.calculated_outputs());
            self.resource_manager.set_consumption(self.building_manager.calculated_upkeeps());

        }

        self.resource_manager.calculate();
        self.upgrade_manager.calculate();
        self.building_manager.calculate(&self.prev_tick_modifiers, &self.resource_manager);

        current_tick_modifiers.combine(self.upgrade_manager.calculated_modifiers());
        current_tick_modifiers.combine(self.resource_manager.calculated_modifiers());
        current_tick_modifiers.combine(self.building_manager.calculated_modifiers());
        
        if self.is_first_tick {

            self.building_manager.calculate(&current_tick_modifiers, &self.resource_manager);
            
        }
        
        self.prev_tick_modifiers = current_tick_modifiers;
        self.is_first_tick = false;
        
    }
    
}

#[derive(Error, Debug)]
pub enum ThingManagerLoadError {
    #[error("wrong load type '{0}' provided")]
    WrongLoadType(String),
    #[error("failed to parse thing")]
    ParseError(#[from]serde_json::Error),
}

impl ThingManager {
    
    pub fn load_from_str(&mut self, load_type: &str, string: &str) -> Result<(), ThingManagerLoadError> {
        
        match load_type {
            "building" => Ok(self.building_manager.load_from_str(string)?),
            "resource" => Ok(self.resource_manager.load_from_str(string)?),
            _ => Err(ThingManagerLoadError::WrongLoadType(load_type.to_string()))
        }
        
    }
    
}
