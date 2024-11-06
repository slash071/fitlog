use chrono::Local;
use fitlog::{valid_input, classify_bmi, display_welcome};
use fitlog::structs::Person;
use colored::Colorize;

// Define reasonable limits for height and weight
const MIN_HEIGHT: f32 = 50.0;
const MAX_HEIGHT: f32 = 300.0;
const MIN_WEIGHT: f32 = 3.0;
const MAX_WEIGHT: f32 = 600.0;

fn main() {
    // Introduction
    display_welcome();

    // Input height and weight from the user
    let current = Local::now();
    let mut user = Person {
        height: valid_input("height", MIN_HEIGHT, MAX_HEIGHT, "cm"),
        weight: valid_input("weight", MIN_WEIGHT, MAX_WEIGHT, "kg"),
        bmi: 0.0,
    };

    // Calculate BMI
    user.bmi = user.calculate_bmi();

    // Print the BMI result
    println!(
        "\n{} {:.2}, {}\n", 
        "Your BMI is:".truecolor(166, 227, 161),
        format!("{}", user.bmi).truecolor(137, 220, 235),
        classify_bmi(user.bmi)
    );

    // Display data
    println!("Saved data: {}, {:.2}, {:.1}cm, {:.1}kg", 
        current.format("%Y-%m-%d"),
        user.bmi,
        user.height,
        user.weight
    );
}
