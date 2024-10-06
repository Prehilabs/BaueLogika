
use std::path::PathBuf;
use crate::core::test_case::TestCase;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Problem
{
    info : ProblemInfo,
    test_cases : Vec<TestCase>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProblemInfo
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

    pub fn get_info(&self) -> &ProblemInfo
    {
        return &self.info;
    }
}