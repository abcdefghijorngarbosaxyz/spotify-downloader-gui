use tauri::ClipboardManager;

#[tauri::command]
pub fn open_about<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>, window: tauri::Window<R>) {
  let package_info: tauri::PackageInfo = app_handle.package_info().clone();
  let webkit_or_webview_version: String = tauri::webview_version().unwrap();
  let os_type: String = os_info::get().os_type().to_string();
  let os_bitness: String = os_info::get().bitness().to_string();
  let os_version: String = os_info::get().version().to_string();

  let message: String = format!(
    r#"Version: {}
Commit: {}
Webkit/WebView: {}
Tauri: {}
OS: {} {} {}
    "#,
    package_info.version,
    std::option_env!("GITHUB_SHA").unwrap_or("N/A"),
    webkit_or_webview_version,
    tauri::VERSION,
    os_type,
    os_bitness,
    os_version,
  );

  tauri::api::dialog::MessageDialogBuilder::new(
    "About ".to_owned() + crate::constants::APP_NAME,
    &message,
  )
  .buttons(
    tauri::api::dialog::MessageDialogButtons::OkCancelWithLabels("Copy".into(), "Close".into()),
  )
  .kind(tauri::api::dialog::MessageDialogKind::Info)
  .parent(&window)
  .show(move |copy: bool| {
    if copy {
      let mut clipboard = app_handle.clipboard_manager();

      clipboard
        .write_text(message)
        .expect("Error while copying to clipboard.");
    }
  });
}

#[derive(serde::Serialize)]
pub struct AppInfo {
  version: String,
  commit: String,
  bit: String,
  platform: String,
}

#[tauri::command]
pub async fn app_info(app: tauri::AppHandle) -> AppInfo {
  let package_info = app.package_info().clone();

  let info: AppInfo = AppInfo {
    version: package_info.version.to_string(),
    commit: std::option_env!("GITHUB_SHA").unwrap_or("N/A").to_string(),
    bit: os_info::get().bitness().to_string(),
    platform: os_info::get().os_type().to_string(),
  };

  info
}
