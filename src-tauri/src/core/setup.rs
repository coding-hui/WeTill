use tauri::App;

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

pub fn init(_app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
