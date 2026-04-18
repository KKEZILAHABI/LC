use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the input file
    let file_path = Path::new("data/students_marks.txt");

    // Read all lines from the file into a string
    let content = fs::read_to_string(file_path)?;
    
    // TODO: Parse each line into integers
    let mut data: Vec<Vec<i32>> = Vec::new();
    
    for line in content.lines(){
        if line.trim().is_empty() {
            continue;
        }
        
        let mut row = Vec::new();
        let mut all_valid = true;
        
    // - Split each line by spaces
    // - Convert each split string into an integer
    // - Store parsed numbers in a vector of integer vectors
for (i, token) in line.split_whitespace().enumerate() {
            match token.parse::<i32>() {
                Ok(num) => row.push(num),
                Err(e) => {
                    eprintln!("Warning: Failed to parse token '{}' at position {} in line '{}', error: {}", 
                             token, i, line, e);
                    all_valid = false;
                    break;  // Stop processing this line
                }
            }
        }
        
        // Only add the row if all tokens were valid
        if all_valid {
            data.push(row);
        }
    }
    
    // Output the parsed table data
    println!("Parsed Table Data:");
    for row in &data {
        println!("{:?}", row);
    }
    
    Ok(())

}