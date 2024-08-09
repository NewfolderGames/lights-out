use crate::core::modifier::ModifierStorage;

pub struct ThingManager {

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
            prev_tick_modifiers: ModifierStorage::new(),
        }

    }

    pub fn tick(&mut self) {
        
        let mut current_tick_modifiers = ModifierStorage::new();
        
        
        
        self.prev_tick_modifiers = current_tick_modifiers;
        
    }
    
}
