use wasm_bindgen::prelude::wasm_bindgen;
use crate::assets::get_asset_list;
use crate::core::{ThingManager, ThingManagerLoadType};

#[wasm_bindgen]
pub struct Game {
    
    thing_manager: ThingManager,
    
    is_playing: bool,
    is_debug_mode: bool,
    
}

// Constructor

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

// Gameplay

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

// Thing loading

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen]
    pub fn load_thing_from_string(&mut self, thing_type: &str, thing: &str) {
        
        let result = match thing_type {
            "building" => self.thing_manager.load_from_str(ThingManagerLoadType::Building, thing),
            _ => Err("wrong thing type".to_string()),
        };
        
        if let Err(message) = result {
            
            eprintln!("thing loader failed to load thing from string, {message}");
            
        } else {
            
            println!("thing loader {thing_type} loaded");
            
        }
        
    }

    #[wasm_bindgen]
    pub fn load_things(&mut self) {
        
        let list = get_asset_list();
        list.iter()
            .for_each(|(thing_type, thing)| {
                self.load_thing_from_string(thing_type, thing)
            });
        
    }
    
}