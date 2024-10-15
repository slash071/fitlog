use std::io;

// Define reasonable limits for height and weight
const MIN_HEIGHT: f32 = 50.0;
const MAX_HEIGHT: f32 = 300.0;
const MIN_WEIGHT: f32 = 3.0;
const MAX_WEIGHT: f32 = 600.0;

pub fn valid_input(measurement: &str) -> f32 {
    if measurement == "height" {
        loop {
            println!("> Enter your height (In cm, between {} to {}): ", MIN_HEIGHT, MAX_HEIGHT);
            
            let mut height_input = String::new();
            io::stdin()
                .read_line(&mut height_input)
                .expect("Failed to read line");
            
            // Try to parse the input as a float
            match height_input.trim().parse::<f32>() {
                Ok(num) if num >= MIN_HEIGHT && num <= MAX_HEIGHT.into() => {
                    return num;
                }
                _ => {
                    println!("Invalid input. Please enter a valid number between {} to {} cm.\n", MIN_HEIGHT, MAX_HEIGHT);
                }
            }
        }
    } else if measurement == "weight" {
        loop {
            println!("> Enter your weight (In kg, between {} to {}): ", MIN_WEIGHT, MAX_WEIGHT);
            
            let mut weight_input = String::new();
            io::stdin()
                .read_line(&mut weight_input)
                .expect("Failed to read line");
            
            // Try to parse the input as a float
            match weight_input.trim().parse::<f32>() {
                Ok(num) if num >= MIN_WEIGHT.into() && num <= MAX_WEIGHT.into() => {
                    return num;
                }
                _ => {
                    println!("Invalid input. Please enter a valid number between {} to {} kg.\n", MIN_WEIGHT, MAX_WEIGHT);
                }
            }
        }
    } else {
        println!(">>ERROR: Invalid argument '{}' for get_valid_input function!<<\nExiting program...", measurement);
        std::process::exit(1);
    }
}

pub fn classify_bmi(bmi: f32) -> &'static str {

    if bmi > 0.0 && bmi < 18.5 { "Underweight" }
    else if bmi >= 18.5 && bmi < 25.0 { "Normal Weight" }
    else if bmi >= 25.0 && bmi < 30.0 { "Overweight" }
    else if bmi >= 30.0 && bmi < 35.0 { "Class I Obesity" }
    else if bmi >= 35.0 && bmi < 40.0 { "Class II Obesity" }
    else { "Class III Obesity" }
}
