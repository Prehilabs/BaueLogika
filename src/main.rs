mod runner;
mod test_case;
mod problem;

use problem::Problem;
use std::collections::HashMap;

fn main() 
{
    println!("Please enter the problem directory:");
    let mut problem_directory = String::new();
    std::io::stdin().read_line(&mut problem_directory).expect("Failed to read line");
    let problem_directory = problem_directory.trim();
    let mut problems = get_problems_from_directory(problem_directory);
    print!("\x1B[2J\x1B[1;1H"); // Clear the screen

    loop
    {
        print_menu(&problems);

        let option = get_users_option();
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen

        if option == 0 {
            break;
        }

        if let Some((_, problem)) = problems.iter_mut().nth(option - 1) {
            problem.print_info();
            if get_confirmation() {
                problem.set_path_to_exe(&get_exe_path());
                print!("\x1B[2J\x1B[1;1H"); // Clear the screen
                problem.run().unwrap();
            }
            
        } else {
            println!("Invalid option. Please try again.");
        }

    }
}

fn print_menu(problem_map : &HashMap<String, Problem>)
{
    println!("Please select a problem to run:");
    println!("0. Exit");
    for (index, problem) in problem_map.iter().enumerate()
    {
        println!("{}. {}", index + 1, problem.0);
    }
}

fn get_confirmation() -> bool
{
    println!("Do you want to run this problem? (y/n)");
    let mut confirmation = String::new();
    std::io::stdin().read_line(&mut confirmation).expect("Failed to read line");
    print!("\x1B[2J\x1B[1;1H"); // Clear the screen
    return confirmation.trim().to_lowercase() == "y";
}

fn get_users_option() -> usize
{
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: usize = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return get_users_option();
        }
    };
    return option;
}

fn get_exe_path() -> String
{
    println!("Please enter the path to the executable:");
    let mut exe_path = String::new();
    std::io::stdin().read_line(&mut exe_path).expect("Failed to read line");
    return exe_path.trim().to_string();
}

fn get_problems_from_directory(problem_directory : &str) -> HashMap<String, Problem>
{
    let paths = std::fs::read_dir(problem_directory).unwrap();
    let mut problems_map = HashMap::new();
    for path in paths
    {
        let path = path.unwrap().path();
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let problem = Problem::from_json_file(path.to_str().unwrap()).unwrap();
        problems_map.insert(file_name, problem);
    }
    return problems_map;
}