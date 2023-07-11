pub mod cmd {
  #[tauri::command]
  pub fn get_platform() -> String {
    os_info::get().os_type().to_string()
  }
}
