#[tauri::command]
pub fn open_about<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>, _window: tauri::Window<R>) {
  let package_info: tauri::PackageInfo = app_handle.package_info().clone();

  let message: String = format!(
    r#"
    {}
    {}

    Version: {}
    Commit: {}
    "#,
    crate::constants::APP_NAME,
    package_info.description,
    package_info.version,
    std::option_env!("GITHUB_SHA").unwrap_or("N/A")
  );

  tauri::api::dialog::MessageDialogBuilder::new("About", message)
    .buttons(tauri::api::dialog::MessageDialogButtons::OkWithLabel(
      "Close".into(),
    ))
    .kind(tauri::api::dialog::MessageDialogKind::Info)
    .show(|_| {});
}
