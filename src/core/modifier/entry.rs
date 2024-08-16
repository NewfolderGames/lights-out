#[derive(Clone)]
pub struct ModifierEntry {
    name: String,
    value: f64,
    calculation: ModifierCalculationMethod,
}

impl ModifierEntry {

    pub fn new(name: String, value: f64, calculation: ModifierCalculationMethod) -> Self {

        Self {
            name,
            value,
            calculation
        }

    }

    pub fn name(&self) -> &str {

        self.name.as_str()

    }
    
    pub fn key(&self) -> String {
        
        format!("{}.{}", self.name, self.calculation.key())
        
    }

    pub fn value(&self) -> f64 {

        self.value

    }

    pub fn set_value(&mut self, value: f64) {

        self.value = value;

    }

    pub fn add_value(&mut self, value: f64) {

        self.value += value;

    }

    pub fn calculation_method(&self) -> ModifierCalculationMethod {

        self.calculation

    }

}

#[derive(Copy, Clone)]
pub enum ModifierCalculationMethod {
    Base,
    Flat,
    Additive,
}

impl ModifierCalculationMethod {
    
    /// Convert `str` to `ModifierCalculationMethod`.
    /// 
    /// # Panic
    /// 
    /// This method will panic if the wrong method is provided.
    pub fn from_str(method: &str) -> ModifierCalculationMethod {

        match method {
            "base" => ModifierCalculationMethod::Base,
            "flat" => ModifierCalculationMethod::Flat,
            "additive" => ModifierCalculationMethod::Additive,
            _ => panic!("wrong modifier calculation method"),
        }
        
    }

    /// Safe version of `from_str` method.
    pub fn from_str_safe(method: &str) -> Result<ModifierCalculationMethod, String> {

        match method {
            "base" => Ok(ModifierCalculationMethod::Base),
            "flat" => Ok(ModifierCalculationMethod::Flat),
            "additive" => Ok(ModifierCalculationMethod::Additive),
            _ => Err("wrong modifier calculation method".to_string()),
        }
        
    }
    
    /// Convert `ModifierCalculationMethod` into a key that can be used as a hashmap's "key".
    pub fn key(&self) -> &'static str {

        match self {
            ModifierCalculationMethod::Base => "base",
            ModifierCalculationMethod::Flat => "flat",
            ModifierCalculationMethod::Additive => "additive",
        }
        
    }
    
}
