use serde::{Serialize, Deserialize};

use crate::test_case::TestCase;
use std::process::{Child, Command, Stdio};

#[derive(Serialize, Deserialize)]
pub struct Runner
{
    path_to_exe : String,
    test_cases : Vec<TestCase>,
    passed_cases : u32
}

impl Runner
{
    pub fn new(path_to_exe: &str, test_cases: Vec<TestCase>) -> Self {
        Runner {
            path_to_exe: String::from(path_to_exe),
            passed_cases : 0,
            test_cases: test_cases
        }
    }

    pub fn print_result(&self)
    {
        println!("Passed {} out of {} test cases", self.passed_cases, self.test_cases.len());
        println!("Success rate: {}%", (self.passed_cases as f32 / self.test_cases.len() as f32) * 100.0);
        if self.passed_cases == self.test_cases.len() as u32
        {
            println!("Congratulations you've solved the problem!");
        }
    }

    pub fn check_test_cases(&mut self) -> Result<(), std::io::Error> {
        for i in 0..self.test_cases.len()
        {
            let test_case = &mut self.test_cases[i];
            let subprocess = Runner::create_subprocess(&self.path_to_exe)?;
            test_case.print_data(i);
            test_case.test(subprocess)?;
            if test_case.get_passed()
            {
                self.passed_cases += 1;
            }
        }
        return Ok(());
    }

    fn create_subprocess(path_to_exe: &str) -> Result<Child, std::io::Error> {
        let process = Command::new(path_to_exe)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        return Ok(process);
    }
}