use crate::runner::Runner;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Problem
{
    pub name : String,
    pub description : String,
    runner : Runner
}

impl Problem 
{
    pub fn from_json_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(file_path)?;
        let problem: Problem = serde_json::from_str(&file_content)?;
        Ok(problem)
    }

    pub fn run(&mut self) -> Result<(), std::io::Error>
    {
        self.runner.check_test_cases()?;
        self.runner.print_result();
        return Ok(());
    }
}