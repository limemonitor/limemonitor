// NOTE: Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dashboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_dashboard() -> serde_json::Value {
    let online = check_internet_connection().await.unwrap_or(false);

    serde_json::json!({
        "online": online,
        "hostname": hostname::get().unwrap_or_default().to_string_lossy().to_string(),
    })
}

async fn check_internet_connection() -> Result<bool, reqwest::Error> {
    const GOOGLE_DNS: &str = "http://8.8.8.8";
    if let Ok(_response) = reqwest::get(GOOGLE_DNS).await {
        Ok(true)
    } else {
        Ok(false)
    }
}
