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
    Addition,
    Multiplicative,
}

impl ModifierCalculationMethod {
    
    pub fn key(&self) -> &'static str {

        match self {
            ModifierCalculationMethod::Base => "base",
            ModifierCalculationMethod::Addition => "addition",
            ModifierCalculationMethod::Multiplicative => "multiplicative",
        }
        
    }
    
}
