use current_locale::current_locale;
use tauri::command;

/// Get the locale of the system.
///
/// # Examples
/// ```
/// use tauri_plugin_locale::get_locale;
///
/// let locale = get_locale().await;
/// ```
#[command]
pub async fn get_locale() -> String {
    current_locale().unwrap_or_else(|_| "en-US".to_string())
}
