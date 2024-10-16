use chrono::Local;
use fitlog::{valid_input, classify_bmi};
use fitlog::structs::Person;

fn main() {
    // Introduction
    let line_length = 86;
    let welcome_text = "Welcome to FitLog";
    let padding = (line_length - welcome_text.len()) / 2;

    println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║ {}{}{} ║", " ".repeat(padding), welcome_text, " ".repeat(padding));
    println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║  This app calculates your Body Mass Index (BMI) and uses the data for visualization.  ║");
    println!("║  * Designed specifically for adults (ages 20 and above).                              ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝\n");

    // Input height and weight from the user
    let current = Local::now();
    let mut user = Person {
        height: valid_input("height"),
        weight: valid_input("weight"),
        bmi: 0.0,
    };

    // Calculate BMI
    user.bmi = user.calculate_bmi();

    // Print the BMI result
    println!("\n───────────────────────────────────");
    println!("Your BMI is: {:.2}, {}", user.bmi, classify_bmi(user.bmi),);
    println!("───────────────────────────────────\n");

    // Display data
    println!("Saved data: {}, {:.2}, {:.1}cm, {:.1}kg", 
        current.format("%Y-%m-%d"),
        user.bmi,
        user.height,
        user.weight
    );
}
