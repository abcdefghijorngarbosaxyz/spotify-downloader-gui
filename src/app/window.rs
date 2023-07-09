pub fn handle_event(event: tauri::GlobalWindowEvent) {
  if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
    let window: tauri::Window = event.window().clone();
    if window.label() == "main" {
      tauri::api::dialog::confirm(
        Some(event.window()),
        "Close ".to_owned() + crate::constants::APP_NAME,
        "Are you sure?",
        move |answer| {
          if answer {
            std::process::exit(0);
          }
        },
      )
    } else {
      window.close().unwrap();
    }
    api.prevent_close();
  }
}
