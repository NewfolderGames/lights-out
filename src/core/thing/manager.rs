use crate::core::modifier::ModifierStorage;
use crate::core::thing::building::BuildingManager;

pub struct ThingManager {

    building_manager: BuildingManager,
    
    prev_tick_modifiers: ModifierStorage,

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
            prev_tick_modifiers: ModifierStorage::new(),
        }

    }

    pub fn tick(&mut self) {
        
        let mut current_tick_modifiers = ModifierStorage::new();
        
        
        
        self.prev_tick_modifiers = current_tick_modifiers;
        
    }
    
}

// Thing loader

pub enum ThingManagerLoadType {
    Building,
}

impl ThingManager {
    
    pub fn load_from_str(&mut self, thing_manager_load_type: ThingManagerLoadType, string: &str) -> Result<(), String> {
        
        match thing_manager_load_type {
            ThingManagerLoadType::Building => self.building_manager.load_from_str(string)
        }
        
    }
    
}