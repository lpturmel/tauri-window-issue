#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use url::Url;

fn main() {
  tauri::Builder::default()
  .setup(|app| {
    let external_url = Url::parse("https://google.com").unwrap();

    app.create_window("window_from_main".to_string(), tauri::WindowUrl::External(external_url), move |window_builder, webview_attributes| {
        (window_builder, webview_attributes)
    });
    Ok(())
  })
  .invoke_handler(tauri::generate_handler![
    spawn_window
  ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn spawn_window(app: tauri::AppHandle) {
  let external_url = Url::parse("https://google.com").unwrap();

  app.create_window("command_window".to_string(), tauri::WindowUrl::External(external_url), move |window_builder, webview_attributes| {
      (window_builder, webview_attributes)
  });
    
}