
use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, Manager};
use std::fs::read_dir;
use crate::core::app_config::{update_problem_path, get_problem_path};
use crate::core::problem::{Problem, ProblemInfo};

#[tauri::command]
pub fn choose_directory(app_handle : AppHandle){
    let app_handle_clone = app_handle.clone();
    FileDialogBuilder::new().pick_folder(move |sel_path|{
        //User selected a directory
        if sel_path.is_some() 
        {
            let selected_path = sel_path.unwrap();
            update_problem_path(app_handle, selected_path);
            //Go to the problem selection page
            let window = app_handle_clone.get_window("main");
            if window.is_some()
            {
                let _ = window.unwrap().eval("window.location.href = 'problemSel.html';");
            }
        }
    });
}

#[tauri::command]
pub fn get_problems_info(app_handle : AppHandle) -> Result<Vec<ProblemInfo>, String> 
{
    let problem_dir = get_problem_path(app_handle);
    let mut problems_info : Vec<ProblemInfo> = Vec::new();
    let dir_files = read_dir(problem_dir).map_err(|e| e.to_string())?;
    
    for file in dir_files
    {
        let file = file.map_err(|e| e.to_string())?;
        let file_path = file.path();
        if file_path.is_file() && file_path.extension() == Some(std::ffi::OsStr::new("json"))
        {
            let problem = Problem::from_json_file(&file_path)?;
            problems_info.push(problem.get_info().clone());
        }
    }
    return Ok(problems_info);
}