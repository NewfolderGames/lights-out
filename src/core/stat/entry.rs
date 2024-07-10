pub struct ModifierEntry {
    name: String,
    value: f64,
    count: i32,
    calculation: ModifierCalculationMethod,
}

impl ModifierEntry {
    
    pub fn new(name: String, value: f64, count: i32, calculation: ModifierCalculationMethod) -> Self {
        
        Self {
            name,
            value,
            count,
            calculation: calculation
        }
        
    }
    
    pub fn get_name(&self) -> &str {
        
        self.name.as_str()
        
    }
    
    pub fn get_value(&self) -> f64 {
        
        self.value
        
    }
    
    pub fn get_calculation_method(&self) -> ModifierCalculationMethod {
        
        self.calculation
        
    }
    
}

#[derive(Copy, Clone)]
pub enum ModifierCalculationMethod {
    Addition,
    Multiplicative,
    Multiply,
}