// Define related data structures
pub struct Person {
    pub height: f32,
    pub weight: f32,
    pub bmi: f32,
}

impl Person {
    pub fn calculate_bmi(&self) -> f32 {
        self.weight / ((self.height / 100.0).powf(2.0))
    }
}