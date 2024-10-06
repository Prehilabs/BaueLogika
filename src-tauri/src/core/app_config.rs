use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    problem_path : String
}

impl AppConfig 
{
    pub fn new() -> Self
    {
        return AppConfig {
            problem_path : String::new()
        };
    }

    pub fn read_from_dir(dir : &PathBuf) -> Self {
        let config_path = dir.join("app.conf");
        if config_path.exists() {
            let file = File::open(config_path).unwrap();
            serde_json::from_reader(file).unwrap()
        } else {
            AppConfig::new()
        }
    }

    pub fn save_in_dir(&self, dir : &PathBuf) -> std::io::Result<()> {
        let config_path = dir.join("app.conf");
        let json_str = to_string(self).unwrap();
        let mut file = File::create(config_path)?;
        file.write_all(json_str.as_bytes())?;
        return Ok(())
    }
}

pub fn update_problem_path(handler : tauri::AppHandle, new_problem_path : String)
{
    let config_path = handler.path_resolver().app_config_dir().unwrap_or_default();

    validate_dir(&config_path);
    let mut config = AppConfig::read_from_dir(&config_path);
    config.problem_path = new_problem_path;
    config.save_in_dir(&config_path).unwrap();
}

pub fn get_problem_path(handler : tauri::AppHandle) -> String
{
    let config_path = handler.path_resolver().app_config_dir().unwrap_or_default();
    let config = AppConfig::read_from_dir(&config_path);
    return config.problem_path;
}


fn validate_dir(dir : &PathBuf)
{
    if !dir.exists()
    {
        std::fs::create_dir_all(&dir).unwrap();
    }
}