use tauri::Manager;
use std::fs;
use std::process::Command as SystemCommand; //alias to avoid clashes with tauri

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// command to scan shortcuts dir and return a list of the filenames
#[tauri::command]
fn get_shortcuts(app: tauri::AppHandle) -> Vec<String> {
  let mut files = Vec::new();

  if let Some(doc_dir) = app.path().document_dir().ok() {
    let target_dir = doc_dir.join("cozy_launcher_shortcuts");

    // read items in dir
    if let Ok(entries) = fs::read_dir(target_dir) {
      for entry in entries.flatten() {
        if let Ok(file_type) = entry.file_type() {
          if file_type.is_file() {
            let filename = entry.file_name().to_string_lossy().into_owned();
            files.push(filename);
          }
        }
      }
    }
  }
  files // returns vector of filenames
}


// command to execute a chosen shortcut when clicked
#[tauri::command]
fn launch_shortcut(app: tauri::AppHandle, filename: String) -> Result<(), String> {
    if let Some(doc_dir) = app.path().document_dir().ok() {
        let shortcut_path = doc_dir.join("cozy_launcher_shortcuts").join(&filename);
        #[cfg(target_os = "windows")]
        {
            // On Windows, use the native cmd shell wrapper to execute shortcuts (.lnk / .url / .exe)
            SystemCommand::new("cmd")
                .args(["/C", "start", "", &shortcut_path.to_string_lossy()])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| e.to_string())?;
        }

        #[cfg(target_os = "macos")]
        {
            // On macOS, use the native 'open' tool to run application shortcuts or scripts
            SystemCommand::new("open")
                .arg(&shortcut_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }

        #[cfg(target_os = "linux")]
        {
            // On Linux, use xdg-open to trigger desktop shortcut files (.desktop)
            SystemCommand::new("xdg-open")
                .arg(&shortcut_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }

        return Ok(());
    }
    Err("Could not locate documents path".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    //NOTE: register commands so that angular can read them
    .invoke_handler(tauri::generate_handler![get_shortcuts, launch_shortcut]) 
    .setup(|app| {
      // NOTE: create shortcuts directory
      if let Some(doc_dir) = app.path().document_dir().ok() {
        let target_dir = doc_dir.join("cozy_launcher_shortcuts");
        match fs::create_dir_all(&target_dir) {
          Ok(_) => println!("created dir successfully! at {:?}", target_dir), //TODO: change for 'match' for "let _ =" and remove Ok and Error cases
          Err(e)=> eprintln!("failed to create dir {}", e),
        }
      }
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
