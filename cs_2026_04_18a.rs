use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the input file
    let file_path = Path::new("data/students_marks.txt");

    // TODO: Read all lines from the file into a string
    let contents = fs::read_to_string(file_path)?;

    // TODO: Split the first line from 'content' into a vector of integers
    let mut data: Vec<i32> = Vec::new();
    if let Some(first_line) = contents.lines().next() {
        
    // - Use whitespace as a delimiter to split        
        for token in first_line.split_whitespace(){
            match token.parse::<i32>(){
    // - Convert the split parts into integers and store in a variable             
                Ok(num) => data.push(num),
                Err(e)=> eprintln!("Error: {}", e)
            }
        }
    }

    // TODO: Print the vector's contents
    // - Use println! to display the vector
    println!("{:?}", data);
    

    Ok(())
}