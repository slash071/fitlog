use fitlog::{
    valid_input,
    classify_bmi,
    display_welcome,
    prompt_yes_no,
    structs::Person,
    db::{open_db, create_table, record_exists, upsert_record}
};
use colored::Colorize;
use rusqlite::Result;

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
        if prompt_yes_no("A record for today already exists. Do you want to overwrite it?") {
            upsert_record(&conn, user.bmi, user.height, user.weight, &classify_bmi(user.bmi, false))?;
            println!("Data for today has been successfully updated!");
        } else {
            println!("No changes made to today's record.");
        }
    } else {
        // Save a new record if no record exists for today
        upsert_record(&conn, user.bmi, user.height, user.weight, &classify_bmi(user.bmi, false))?;
        println!("Data for today has been successfully saved!");
    }

    Ok(())
}
