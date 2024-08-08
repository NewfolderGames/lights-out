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
            calculation
        }

    }

    pub fn name(&self) -> &str {

        self.name.as_str()

    }
    
    pub fn key(&self) -> String {
        
        let mut key = self.name.clone();
        
        match self.calculation {
            ModifierCalculationMethod::Addition => key.push_str(".addition"),
            ModifierCalculationMethod::Multiplicative => key.push_str(".multiplicative"),
            ModifierCalculationMethod::Multiply => key.push_str(".multiply"),
        }
        
        key
        
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

    pub fn count(&self) -> i32 {

        self.count

    }

    pub fn set_count(&mut self, count: i32) {

        self.count = count;

    }

    pub fn add_count(&mut self, count: i32) {

        self.count += count;

    }

}

#[derive(Copy, Clone)]
pub enum ModifierCalculationMethod {
    Addition,
    Multiplicative,
    Multiply,
}
