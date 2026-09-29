mod process;



use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn kill_process(app:AppHandle, pid: u32) -> Result<String, String> {
    process::kill_process(app, pid)
}

#[tauri::command]
async fn get_processes() -> Result<Vec<process::PortInfo>, String> {
    process::fetch_processes()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
   tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![kill_process, get_processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}