use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, Manager};
use crate::core::app_config::update_problem_path;

#[tauri::command]
pub fn choose_directory(app_handle : AppHandle){
    let app_handle_clone = app_handle.clone();
    FileDialogBuilder::new().pick_folder(move |sel_path|{
        //User selected a directory
        if sel_path.is_some() 
        {
            let selected_path = sel_path.unwrap();
            let selected_path_str = selected_path.to_str().unwrap().to_string();
            update_problem_path(app_handle, selected_path_str);
            let window = app_handle_clone.get_window("main");
            if window.is_some(){
                let cmd = window.unwrap().eval("window.location.href = 'problemSel.html';");
            }
        }
    });
}