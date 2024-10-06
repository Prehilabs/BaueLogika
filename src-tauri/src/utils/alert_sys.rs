use tauri::api::dialog::{MessageDialogBuilder, MessageDialogKind, MessageDialogButtons};

pub fn log_error(message : &str)
{
    MessageDialogBuilder::new("Error", message)
    .kind(MessageDialogKind::Error)
    .buttons(MessageDialogButtons::Ok)
    .show(|_| {});
}