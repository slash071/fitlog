use std::io::{self, Write};
use colored::Colorize;

pub mod structs;
pub mod db;

// Display welcome message
pub fn display_welcome() {
    let title = "Welcome to FitLog";
    let intro = "This app calculates your Body Mass Index (BMI) and uses the data for visualization.";
    let intro2 = "* Designed specifically for adults (ages 20 and above).";
    let line_length =intro.len() + 4;

    println!("╔{}╗", "═".repeat(line_length));
    println!("║{}║", format!("{:^1$}", title.truecolor(245, 169, 127).bold(), line_length));
    println!("╠{}╣", "═".repeat(line_length));
    println!("║{}║", format!("{:^1$}", intro.truecolor(138, 173, 244), line_length));
    println!("║{}║", format!("{:^1$}", intro2.truecolor(138, 173, 244), line_length));
    println!("╚{}╝\n", "═".repeat(line_length));
}

// Validate user input for height or weight
pub fn valid_input(measurement: &str, min: f64, max: f64, unit: &str) -> f64 {
    loop {
        print!("{}",
            format!(
                "> Enter your {} (between {} and {} {}): ",
                measurement, min, max, unit
            ).truecolor(203, 166, 247)
        );
        io::stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) if num >= min && num <= max => return num,
            _ => println!("{}",
                format!(
                    "Invalid input! Please enter a valid {} between {} and {} {}.\n",
                    measurement, min, max, unit
                ).truecolor(243, 139, 168)  // Pinkish-red for error
            ),
        }
    }
}

// Classify BMI into categories
pub fn classify_bmi(bmi: f64, color: bool) -> String {
    let (category, color_code) = match bmi {
        bmi if bmi < 18.5 => ("Underweight", (250, 179, 135)),
        bmi if bmi < 25.0 => ("Normal Weight", (166, 227, 161)),
        bmi if bmi < 30.0 => ("Overweight", (245, 194, 231)),
        bmi if bmi < 35.0 => ("Class I Obesity", (250, 179, 135)),
        bmi if bmi < 40.0 => ("Class II Obesity", (243, 139, 168)),
        _ => ("Class III Obesity", (242, 143, 173)),
    };
    
    if color {
        category.truecolor(color_code.0, color_code.1, color_code.2).to_string()
    } else {
        category.to_string()
    }
}
