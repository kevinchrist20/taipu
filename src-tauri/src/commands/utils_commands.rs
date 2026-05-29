use std::{fs, path::PathBuf};

const SETTINGS_DIR_NAME: &str = "taipu";
const SETTINGS_FILE_NAME: &str = "settings.ini";
const DEFAULT_THEME: &str = "LIGHT";

fn normalize_theme(theme: &str) -> String {
    theme.trim().to_uppercase()
}

fn is_valid_theme(theme: &str) -> bool {
    matches!(theme, "LIGHT" | "DARK")
}

fn settings_file_path() -> Result<PathBuf, String> {
    let mut dir_path = dirs::config_dir().ok_or("Unable to resolve config directory.")?;
    dir_path.push(SETTINGS_DIR_NAME);

    fs::create_dir_all(&dir_path)
        .map_err(|e| format!("Unable to create settings directory: {:?}", e))?;

    dir_path.push(SETTINGS_FILE_NAME);
    Ok(dir_path)
}

fn parse_theme(contents: &str) -> Option<String> {
    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        if let Some(value) = trimmed.strip_prefix("theme=") {
            let theme = normalize_theme(value);
            if is_valid_theme(&theme) {
                return Some(theme);
            }
        }
    }

    None
}

fn serialize_theme(theme: &str) -> String {
    format!("[appearance]\ntheme={}\n", theme)
}

#[tauri::command]
pub fn exit_app() {
    std::process::exit(0x0);
}

#[tauri::command]
pub fn get_app_theme() -> Result<String, String> {
    let path = settings_file_path()?;

    if !path.exists() {
        return Ok(DEFAULT_THEME.to_string());
    }

    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Unable to read settings file: {:?}", e))?;

    Ok(parse_theme(&contents).unwrap_or_else(|| DEFAULT_THEME.to_string()))
}

#[tauri::command]
pub fn set_app_theme(theme: String) -> Result<(), String> {
    let normalized_theme = normalize_theme(&theme);

    if !is_valid_theme(&normalized_theme) {
        return Err("Unsupported theme value.".to_string());
    }

    let path = settings_file_path()?;
    let contents = serialize_theme(&normalized_theme);

    fs::write(path, contents).map_err(|e| format!("Unable to save settings file: {:?}", e))
}
