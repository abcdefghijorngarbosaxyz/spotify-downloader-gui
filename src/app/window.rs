pub fn handle_event(event: tauri::GlobalWindowEvent) {
  if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
    let window: tauri::Window = event.window().clone();
    if window.label() == "main" {
      tauri::api::dialog::MessageDialogBuilder::new(
        "Close ".to_owned() + crate::constants::APP_NAME,
        "Are you sure?",
      )
      .buttons(tauri::api::dialog::MessageDialogButtons::YesNo)
      .parent(&window)
      .kind(tauri::api::dialog::MessageDialogKind::Warning)
      .show(|yes: bool| {
        if yes {
          std::process::exit(0);
        }
      });
    } else {
      window.close().unwrap();
    }
    api.prevent_close();
  }
}

pub mod cmd {
  use tauri::Manager;

  #[tauri::command]
  pub fn close_window(app: tauri::AppHandle) {
    let window: tauri::Window = app.get_window("main").unwrap().clone();
    tauri::api::dialog::MessageDialogBuilder::new(
      "Close ".to_owned() + crate::constants::APP_NAME,
      "Are you sure?",
    )
    .buttons(tauri::api::dialog::MessageDialogButtons::YesNo)
    .parent(&window)
    .kind(tauri::api::dialog::MessageDialogKind::Warning)
    .show(|yes: bool| {
      if yes {
        std::process::exit(0);
      }
    });
  }
}
