use chrono::Local;
use fitlog::{valid_input, classify_bmi};

fn main() {
    // Introduction
    let line_length = 86;
    let welcome_text = "Welcome to the BMI Tracker App for Adults";
    let padding = (line_length - welcome_text.len()) / 2;

    println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║ {}{}{} ║", " ".repeat(padding), welcome_text, " ".repeat(padding));
    println!("╠═══════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║  This app calculates your Body Mass Index (BMI) and uses the data for visualization.  ║");
    println!("║  * Note: This app is intended for adults only (aged 20 and above).                    ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝\n");

    // Input height and weight
    let height = valid_input("height");
    let weight = valid_input("weight");
    let current = Local::now();

    // BMI calculation
    let height_m: f32 = height / 100.0;  // Convert height to meters
    let bmi: f32 = weight / height_m.powf(2.0);

    // Print result
    println!("\n───────────────────────────────────");
    let bmi_category = classify_bmi(bmi);
    println!("Your BMI is: {bmi:.2}, {bmi_category}");
    println!("───────────────────────────────────\n");

    println!("\nSaved data: {}, {:.2}, {:.1}cm, {:.1}kg", current.format("%Y-%m-%d"), bmi, height, weight);
}
