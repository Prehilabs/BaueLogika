mod runner;
mod test_case;
mod problem;

use problem::Problem;

fn main() 
{
    let mut problem = Problem::from_json_file("C:\\Users\\nanib\\Escritorio\\test.json").unwrap();

    println!("Problem name: {}", problem.name);
    println!("Description: {}", problem.description);
    problem.run().unwrap();
}
