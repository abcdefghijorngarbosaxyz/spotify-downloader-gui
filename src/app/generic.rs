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
}
