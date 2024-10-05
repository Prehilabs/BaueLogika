use std::{path, sync::{Arc, Mutex}};
use tauri::api::dialog::FileDialogBuilder;
use crate::utils::problem_reader::ProblemReader;

#[tauri::command]
pub async fn choose_directory() {
    let choosen_dir : Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    FileDialogBuilder::new().pick_folder(move |sel_path|{
        //User selected a directory
        if sel_path.is_some() 
        {
            let path = sel_path.unwrap();
            *choosen_dir.lock().unwrap() = String::from(path.to_str().unwrap_or_default());
        }

        //Create instance of ProblemReader
        if *choosen_dir.lock().unwrap() != String::new()
        {
            let problem_reader = ProblemReader::new(&*choosen_dir.lock().unwrap());
            println!("{:?}", problem_reader.get_problem_files());
        }
    });
}
