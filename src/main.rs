mod runner;
mod test_case;
mod problem;

use problem::Problem;

fn main() 
{
    let mut problem = Problem::from_json_file("D:\\Programacion\\BaueLogika\\src\\Test.json").unwrap();

    problem.print_info();
    problem.set_path_to_exe("C:\\Users\\nanib\\Escritorio\\traza_peano.exe");
    problem.run().unwrap();
}
