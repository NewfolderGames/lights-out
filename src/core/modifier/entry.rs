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

    pub fn get_name(&self) -> &str {

        self.name.as_str()

    }

    pub fn get_value(&self) -> f64 {

        self.value

    }

    pub fn get_calculation_method(&self) -> ModifierCalculationMethod {

        self.calculation

    }

    pub fn get_count(&self) -> i32 {

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
