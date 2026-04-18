use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("data/moon_temp.txt");
    let content = fs::read_to_string(file_path)?;

    let mut day_temp: Vec<Vec<i32>> = Vec::new();

    for line in content.lines() {
        let mut row: Vec<i32> = Vec::new();

        for token in line.split_whitespace() {
            match token.parse::<i32>() {
                Ok(num) => row.push(num),
                Err(_) => continue, // skip invalid values
            }
        }

        if !row.is_empty() {
            day_temp.push(row);
        }
    }

    // Calculate and print averages
    for (i, day) in day_temp.iter().enumerate() {
        let sum: i32 = day.iter().sum();
        let avg = sum as f64 / day.len() as f64;

        println!("Day {} average temperature: {:.2}", i + 1, avg);
    }

    Ok(())
}