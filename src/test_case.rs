use std::process::{Child, ChildStdin, ChildStdout};
use std::io::{Read, Write};


pub struct TestCase
{
    inputs: String,
    expected: String,
    passed : bool,
    hidden : bool
} 


impl TestCase
{
    pub fn new(inputs: &str, expected: &str, hidden : bool) -> TestCase {
        TestCase 
        { 
            inputs: String::from(inputs), 
            expected : String::from(expected),
            passed : false,
            hidden : hidden
        }
    }

    pub fn get_passed(&self) -> bool
    {
        return self.passed;
    }

    pub fn print_data(&self, test_index : usize)
    {
        println!("Test case {}", test_index);
        if !self.hidden
        {
            println!("Inputs: [{}]", self.inputs.replace("\n", ", "));
            println!("Expected output: {}", self.expected.replace("\n", ", "));
        }
        else
        {
            println!("Hidden Test");
        }
    }

    pub fn test(&mut self, mut process : Child) -> Result<(), std::io::Error>
    {
        let stdin = process.stdin.as_mut().unwrap();
        self.write_inputs(stdin).unwrap();
        
        process.wait()?;

        let stdout = process.stdout.as_mut().unwrap();
        let outputs = self.read_outputs(stdout).unwrap();

        if !self.hidden
        {
            println!("Your outputs: [{}]", outputs);
        }

        self.passed = self.expected == outputs;

        println!("Test result: {}Passed\n", if self.passed { "" } else { "Not " });

        return Ok(())
    }

    fn write_inputs(&self, stdin :&mut ChildStdin) -> Result<(), std::io::Error> {
        stdin.write_all(self.inputs.as_bytes())?;
        stdin.write(b"\n")?;
            
        return Ok(());
    }

    fn read_outputs(&self, stdout :&mut ChildStdout) -> Result<String, std::io::Error> 
    {
        let mut buffer = String::new();
        stdout.read_to_string(&mut buffer)?;
        return Ok(buffer);
    }
}