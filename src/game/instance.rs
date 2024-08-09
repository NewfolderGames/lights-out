use wasm_bindgen::prelude::wasm_bindgen;
use crate::core::ThingManager;

#[wasm_bindgen]
pub struct Game {
    
    thing_manager: ThingManager,
    
    is_playing: bool,
    is_debug_mode: bool,
    
}

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        // Setup panic hook.

        console_error_panic_hook::set_once();

        // Return

        Self {
            thing_manager: ThingManager::new(),
            is_playing: false,
            is_debug_mode: false,
        }

    }

}

impl Default for Game {

    fn default() -> Self {

        Self::new()

    }

}

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen]
    pub fn pause(&mut self) {

        self.is_playing = false;

    }

    #[wasm_bindgen]
    pub fn resume(&mut self) {

        self.is_playing = true;

    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {

        if !self.is_playing { return }
        
        

    }
    
    #[wasm_bindgen]
    pub fn rerender(&self) {
        
        
        
    }

    #[wasm_bindgen]
    pub fn set_debug_mode(&mut self, active: bool) {

        self.is_debug_mode = active;

    }

}
