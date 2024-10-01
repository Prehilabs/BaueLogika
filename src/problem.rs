use crate::runner::Runner;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Problem
{
    name : String,
    description : String,
    example_case : [String; 2],
    runner : Runner
}

impl Problem 
{
    pub fn from_json_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(file_path)?;
        let problem: Problem = serde_json::from_str(&file_content)?;
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

    pub fn set_path_to_exe(&mut self, path_to_exe: &str)
    {
        self.runner.set_path_to_exe(path_to_exe);
    }

    pub fn run(&mut self) -> Result<(), std::io::Error>
    {
        self.runner.check_test_cases()?;
        self.runner.print_result();
        return Ok(());
    }
}