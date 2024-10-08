
use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, Manager};
use std::fs::read_dir;
use crate::core::app_config::{update_problem_path, get_problem_path};
use crate::core::problem::{Problem, ProblemInfo};
use crate::core::test_case::TestCase;
use crate::utils::alert_sys::log_error;
use crate::utils::validation::{problem_path_is_valid, exe_path_is_valid};

#[tauri::command]
pub fn choose_directory(app_handle : AppHandle){
    let app_handle_clone = app_handle.clone();
    FileDialogBuilder::new().pick_folder(move |sel_path|{
        //User selected a directory
        if sel_path.is_some() 
        {
            let selected_path = &sel_path.unwrap();
            
            //Verify permissions for the selected directory
            if !problem_path_is_valid(selected_path)
            {
                log_error("Invalid problem path");
                return;
            }

            //Show error message if updating the problem path failed
            if update_problem_path(app_handle, selected_path).is_err()
            {
                log_error("Error updating the problem path");
                return;
            }

            //Go to the problem selection page
            let window = app_handle_clone.get_window("main");
            if window.is_none() || window.unwrap().eval("window.location.href = 'problemSel.html';").is_err()
            {
                log_error("Error opening problem selection page");
                return;
            }
        }
    });
}

#[tauri::command]
pub fn get_problems_info(app_handle : AppHandle) -> Result<Vec<ProblemInfo>, String> 
{
    let problem_dir = get_problem_path(app_handle).map_err(|e| e.to_string())?;

    //Dont require validation as the path is already validated
    let dir_files = read_dir(problem_dir).unwrap();
    
    let mut problems_info : Vec<ProblemInfo> = Vec::new();

    for file in dir_files
    {
        let file_path = file.map_err(|e| e.to_string())?.path();
        if file_path.is_file() && file_path.extension() == Some(std::ffi::OsStr::new("json"))
        {
            //Only add the problem info if the problem is successfully deserialized
            let problem = Problem::from_json_file(&file_path);
            if problem.is_ok()
            {
                problems_info.push(problem.unwrap().get_info().clone());
            }
        }
    }

    if problems_info.is_empty()
    {
        return Err("No problems found".to_string());
    }
    else 
    {
        return Ok(problems_info);     
    }
}

#[tauri::command]
pub fn load_problem(app_handle : AppHandle, problem_name : String) -> Result<Problem, String>
{
    let problem_dir = get_problem_path(app_handle).map_err(|e| e.to_string())?;
    let problem_path = problem_dir.join(format!("{}.json", problem_name.replace(" ", "_")));
    let problem = Problem::from_json_file(&problem_path).map_err(|e| e.to_string())?;

    return Ok(problem);
}

#[tauri::command]
pub fn run_problem(problem: String, exe_path : String, test_cases : Vec<TestCase>)
{
    if exe_path_is_valid(&exe_path.into())
    {
        //Run the problem
    }
    else
    {
        log_error("Invalid exe path");
    }
    
}