mod runner;
mod test_case;
mod problem;

use problem::Problem;
use std::collections::HashMap;
use std::path::Path;

fn main() 
{
    let mut problems;
    loop {
        println!("Please enter the problem directory:");
        let mut problem_directory = String::new();
        std::io::stdin().read_line(&mut problem_directory).expect("Failed to read line");
        let problem_directory = problem_directory.trim();

        if Path::new(problem_directory).exists() {
            problems = get_problems_from_directory(problem_directory);
            print!("\x1B[2J\x1B[1;1H"); // Clear the screen
            break;
        } else {
            println!("The directory does not exist. Please try again.");
        }
    }

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
                println!("Press Enter to continue...");
                std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
                print!("\x1B[2J\x1B[1;1H"); // Clear the screen
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
    println!("Do you want to try this problem? (y/n)");
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
    loop {
        println!("Please enter the path to the executable:");
        let mut exe_path = String::new();
        std::io::stdin().read_line(&mut exe_path).expect("Failed to read line");
        let exe_path = exe_path.trim().to_string();

        let path = Path::new(&exe_path);

        if path.exists() && path.extension() == Some(std::ffi::OsStr::new("exe")) {
            return exe_path;
        } else {
            println!("The path does not exist or is not an executable. Please try again.");
        }
    }
}

fn get_problems_from_directory(problem_directory : &str) -> HashMap<String, Problem>
{
    let paths = std::fs::read_dir(problem_directory).unwrap();
    let mut problems_map = HashMap::new();
    for path in paths
    {
        let path = path.unwrap().path();
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_string().replace("_", "");
        if path.extension() == Some(std::ffi::OsStr::new("json"))
        {
            let problem_result = Problem::from_json_file(path.to_str().unwrap());
            if problem_result.is_ok()
            {
                problems_map.insert(file_name, problem_result.unwrap());
            }
            
        }
    }
    return problems_map;
}