mod runner;
mod test_case;
mod problem;

use problem::Problem;

fn main() 
{
    let mut problem = Problem::from_json_file("D:\\Programacion\\BaueLogika\\problems\\Count_vowels.json").unwrap();
    problem.print_info();
}
