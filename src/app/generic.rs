pub mod cmd {
  use tauri::Manager;

  #[tauri::command]
  pub fn get_platform() -> String {
    os_info::get().os_type().to_string()
  }

  #[tauri::command]
  pub fn join_us_on_discord(app: tauri::AppHandle) {
    tauri::api::shell::open(
      &app.app_handle().shell_scope(),
      crate::constants::DISCORD_URL,
      None,
    )
    .unwrap();
  }

  #[tauri::command]
  pub fn report_issue(app: tauri::AppHandle) {
    tauri::api::shell::open(
      &app.app_handle().shell_scope(),
      crate::constants::ISSUES_URL,
      None,
    )
    .unwrap();
  }

  #[tauri::command]
  pub fn docs(app: tauri::AppHandle) {
    tauri::api::shell::open(
      &app.app_handle().shell_scope(),
      crate::constants::DOCS_URL,
      None,
    )
    .unwrap();
  }

  #[tauri::command]
  pub fn devtools(_app: tauri::AppHandle, window: tauri::Window) {
    if window.is_devtools_open() {
      window.close_devtools();
    } else {
      window.open_devtools();
    }
  }

  #[tauri::command]
  pub fn minimize(_app: tauri::AppHandle, window: tauri::Window) {
    window.minimize().unwrap();
  }

  #[tauri::command]
  pub async fn always_on_top(_app: tauri::AppHandle, window: tauri::Window) -> bool {
    let app_config: crate::config::AppConfig = crate::config::AppConfig::read().await;

    let always_on_top: bool = !app_config.always_on_top;
    window.set_always_on_top(always_on_top).unwrap();
    app_config
      .patch(serde_json::json!({ "always_on_top": always_on_top }))
      .write()
      .await;
    always_on_top
  }

  #[tauri::command]
  pub async fn is_always_on_top(_app: tauri::AppHandle, _window: tauri::Window) -> bool {
    let app_config: crate::config::AppConfig = crate::config::AppConfig::read().await;

    app_config.always_on_top
  }

  #[tauri::command]
  pub async fn open_download_folder(app: tauri::AppHandle, window: tauri::Window) {
    let app_config: crate::config::AppConfig = crate::config::AppConfig::read().await;

    if crate::utils::path_exists(std::path::PathBuf::from(&app_config.download_folder).as_path()) {
      tauri::api::shell::open(&app.shell_scope(), &app_config.download_folder, None).unwrap();
    } else {
      tauri::api::dialog::MessageDialogBuilder::new("Folder Not Found", "Select another folder.")
        .buttons(tauri::api::dialog::MessageDialogButtons::Ok)
        .parent(&window)
        .kind(tauri::api::dialog::MessageDialogKind::Error)
        .show(|_| {});
    }
  }

  #[tauri::command]
  pub async fn select_download_folder(_app: tauri::AppHandle) {
    let app_config: crate::config::AppConfig = crate::config::AppConfig::read().await;

    tokio::task::spawn(async move {
      let folder: Option<std::path::PathBuf> =
        tauri::api::dialog::blocking::FileDialogBuilder::new().pick_folder();
      if folder.is_some() {
        let folder_path: String = folder.unwrap().to_str().unwrap().to_string();
        app_config
          .patch(serde_json::json!({ "download_folder": folder_path }))
          .write()
          .await;
        log::info!("Download folder changed: {}", folder_path);
      }
    })
    .await
    .unwrap();
  }
}
