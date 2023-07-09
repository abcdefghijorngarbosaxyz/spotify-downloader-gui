use tauri::ClipboardManager;

#[tauri::command]
pub fn open_about<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>, window: tauri::Window<R>) {
  let package_info: tauri::PackageInfo = app_handle.package_info().clone();
  let webkit_or_webview_version: String = tauri::webview_version().unwrap();

  let message = format!(
    r#"Version: {}
Commit: {}
Webkit/WebView: {}
Tauri: {}
    "#,
    package_info.version,
    std::option_env!("GITHUB_SHA").unwrap_or("N/A"),
    webkit_or_webview_version,
    tauri::VERSION
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
  .show(move |copy| {
    if copy {
      let mut clipboard = app_handle.clipboard_manager();

      clipboard
        .write_text(message)
        .expect("Error while copying to clipboard.");
    }
  });
}
