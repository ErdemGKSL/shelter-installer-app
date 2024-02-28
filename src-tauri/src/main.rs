#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use platform_dirs::AppDirs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn toggle_inject(discord_type: &str) -> Result<(), String> {
    return toggle_inject_raw(discord_type.into()).await
}

#[tauri::command]
async fn update(discord_type: &str) -> Result<(), String> {
    let discord_type: Type = discord_type.into();
    let (resources, _) = get_resources_folder(discord_type.clone(), true).or(get_resources_folder(discord_type, false))?;

    let app_path = resources.join("app");

    inject_raw(resources, app_path).await
}


#[tauri::command]
fn is_installed(discord_type: &str) -> (bool, bool) {
    if let Ok((_, installed)) = get_resources_folder(discord_type.into(), true).or(get_resources_folder(discord_type.into(), false)) {
        (true, installed)
    } else {
        (false, false)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, is_installed, toggle_inject, update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
enum Type {
  STD,
  PTB,
  CAN
}

impl Type {
  pub fn as_dirname(&self) -> String {
    match self {
      Type::STD => "Discord".to_string(),
      Type::PTB => "DiscordPTB".to_string(),
      Type::CAN => "DiscordCanary".to_string()
    }
  }
}

impl From<&str> for Type {
  fn from(s: &str) -> Self {
    match s.to_lowercase().as_str() {
      "ptb" | "discordptb" | "p" => Self::PTB,
      "can" | "canary" | "c" => Self::CAN,
      _ => Self::STD,
    }
  }
}

const DOWNLOAD_FILES: [&str; 3] = [
  "https://raw.githubusercontent.com/uwu/shelter/main/injectors/desktop/app/index.js",
  "https://raw.githubusercontent.com/uwu/shelter/main/injectors/desktop/app/preload.js",
  "https://raw.githubusercontent.com/uwu/shelter/main/injectors/desktop/app/package.json"
];

async fn inject_raw(resources: PathBuf, app_path: PathBuf) -> Result<(), String> {

    let app_asar_path = resources.join("app.asar");

    if app_asar_path.exists() {
        std::fs::rename(
            resources.join("app.asar"),
            resources.join("original.asar")
        ).map_err(|_| "Couldn't rename the app.asar in the resources while installing.")?;
    }
    
    std::fs::create_dir_all(&app_path)
        .map_err(|_| "Couldn't create app directory while installing.")?;

    for file_url in DOWNLOAD_FILES {
        let file_name = file_url.split("/").last().unwrap();

        download_file(app_path.join(file_name), file_url.to_string()).await?;
    }
    Ok(())
}

async fn toggle_inject_raw(discord_type: Type) -> Result<(), String> {
  let (resources, installed) = get_resources_folder(discord_type.clone(), true).or(get_resources_folder(discord_type, false))?;

  let app_path = resources.join("app");

  if installed {
    std::fs::rename(
      resources.join("original.asar"),
      resources.join("app.asar")
    ).map_err(|_| "Couldn't rename the original.asar in the resources while uninstalling.")?;

    if app_path.exists() {
        std::fs::remove_dir_all(app_path).map_err(|_| "Couldn't remove app folder in the resources while uninstalling.")?;
    }

    return Ok(());
  }

  inject_raw(resources, app_path).await
}

async fn download_file(path: PathBuf, url: String) -> Result<(), String> {
  let content = reqwest::get(url.clone())
      .await
      .map_err(|_| format!("Couldn't fetch data from {url}"))?
      .text()
      .await
      .map_err(|_| format!("Couldn't parse data from {url}"))?;

  std::fs::write(&path, content)
      .map_err(|_| format!("Couldn't write data to {}", path.display()))?;

  Ok(())
}

fn get_data_dir(xdg: bool) -> Result<PathBuf, String> {
  let user_dirs = AppDirs::new(None, xdg);
  Ok(user_dirs.ok_or("Couldn't find data directory")?.data_dir)
}

fn get_resources_folder(discord_type: Type, xdg: bool) -> Result<(PathBuf, bool), String> {
  let app_data = get_data_dir(xdg)?;

  let app_data_path = app_data.as_path();
  let app_path = app_data_path.join(discord_type.as_dirname());

  let app_path = app_path.as_path();
  
  let dir = std::fs::read_dir(app_path).map_err(|_| "Couldn't read the directory")?;
  
  let dir = dir
      .filter(|entry| {
        if let Ok(entry) = entry {
          entry.file_name().to_str().unwrap_or("").to_string().starts_with("app-1.") &&
              (
                entry.path().join("resources/app.asar").exists() ||
                    entry.path().join("resources/original.asar").exists()
              )
        } else {
          false
        }
      })
      .map(|entry| {
        let entry = entry.unwrap();
        entry.path()
      })
      .max()
      .ok_or("Couldn't find valid app version directory with asar file")?;

  let resources_dir = dir.join("resources");
  let is_installed = resources_dir.join("original.asar").exists();

  Ok((
    resources_dir,
    is_installed
  ))
}
