
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Problem
{
    name : String,
    description : String,
    example_case : [String; 2]
}

impl Problem 
{
    pub fn from_json_file(file_path: &PathBuf) -> Result<Self, String> {
        let file_content = std::fs::read_to_string(file_path).map_err(|e|e.to_string())?;
        let problem: Problem = serde_json::from_str(&file_content).map_err(|e|e.to_string())?;
        Ok(problem)
    }

    pub fn print_info(&self)
    {
        println!("{} Problem", self.name);
        println!("Description:\n{}", self.description);
        println!("Example:");
        println!("Input:\n{}\n", self.example_case[0]);
        println!("Expected output:\n{}\n", self.example_case[1]);
    }
}