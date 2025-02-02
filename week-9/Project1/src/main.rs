use std::fs::File;
use std::io::{self, Write};

// Define drink categories
const DRINK_CATEGORIES: [&str; 4] = [
    "Lager",
    "Stout",
    "Non-alcoholic",
    "Spirit",
];

// Function to save categories to a file
fn save_drink_categories_to_file(filename: &str) -> io::Result<()> {
    // Open or create the file
    let mut file = File::create(filename)?;

    // Write a header line (optional)
    writeln!(file, "Drink Categories for Nigerian Breweries Plc")?;

    // Write each category to the file
    for category in DRINK_CATEGORIES.iter() {
        writeln!(file, "{}", category)?;
    }

    Ok(())
}

fn main() {
    // Ask user for the filename
    println!("Enter the filename to save drink categories (e.g., categories.txt): ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim(); // Remove newline characters

    // Call the function to save the categories to the file
    match save_drink_categories_to_file(filename) {
        Ok(_) => println!("Drink categories saved to '{}'", filename),
        Err(e) => println!("Error saving to file: {}", e),
    }
}
