use fitlog::{
    valid_input,
    classify_bmi,
    display_welcome,
    structs::Person,
    db::{open_db, create_table, record_exists, upsert_record}
};
use colored::Colorize;
use rusqlite::Result;
use std::io::{self, Write};

const MIN_HEIGHT: f64 = 50.0;
const MAX_HEIGHT: f64 = 300.0;
const MIN_WEIGHT: f64 = 3.0;
const MAX_WEIGHT: f64 = 600.0;

fn main() -> Result<()> {
    // Introduction
    display_welcome();

    // Input height and weight from the user
    let mut user = Person {
        height: valid_input("height", MIN_HEIGHT, MAX_HEIGHT, "cm"),
        weight: valid_input("weight", MIN_WEIGHT, MAX_WEIGHT, "kg"),
        bmi: 0.0,
    };

    // Calculate BMI
    user.bmi = user.calculate_bmi();

    // Print the BMI result
    println!(
        "\n{} {}, {}\n", 
        "Your BMI is:".truecolor(166, 227, 161),
        format!("{}", user.bmi).truecolor(137, 220, 235),
        classify_bmi(user.bmi, true)
    );

    // Open the SQLite database connection
    let conn = open_db()?;
    create_table(&conn)?;

    // Check if a record already exists for today
    if record_exists(&conn)? {
        // Prompt the user for overwrite confirmation
        print!("A record for today already exists. Do you want to overwrite it? (yes/no): ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");

        match answer.trim().to_lowercase().as_str() {
            "yes" => {
                upsert_record(&conn, user.bmi, user.height, user.weight, &classify_bmi(user.bmi, false))?;
                println!("Data for today has been successfully updated!");
            }
            "no" => {
                println!("No changes made to today's record.");
            }
            _ => {
                println!("Invalid response. No changes made to today's record.");
            }
        }        
    } else {
        // Insert a new record if no record exists for today
        upsert_record(&conn, user.bmi, user.height, user.weight, &classify_bmi(user.bmi, false))?;
        println!("Data for today has been successfully saved!");
    }

    Ok(())
}
