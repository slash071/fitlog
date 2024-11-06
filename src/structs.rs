// Define related data structures
pub struct Person {
    pub height: f64,
    pub weight: f64,
    pub bmi: f64,
}

impl Person {
    pub fn calculate_bmi(&self) -> f64 {
        let bmi = self.weight / ((self.height / 100.0).powf(2.0));
        (bmi * 100.0).round() / 100.0  // Round to two decimal places
    }
}
