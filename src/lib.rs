use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

pub use commands::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("locale")
        .invoke_handler(tauri::generate_handler![commands::get_locale])
        .build()
}
