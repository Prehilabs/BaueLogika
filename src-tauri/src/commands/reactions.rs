use crate::utils::alert_sys::log_error;

#[tauri::command]
pub fn show_error(message : String)
{
    log_error(&message);
}