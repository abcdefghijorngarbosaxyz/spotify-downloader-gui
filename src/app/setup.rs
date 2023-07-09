use tauri::Manager;

use crate::constants;

pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
  let app_handle: tauri::AppHandle = app.app_handle();

  tauri::async_runtime::spawn(async move {
    let main_window: tauri::WindowBuilder<'_> =
      tauri::WindowBuilder::new(&app_handle, "main", tauri::WindowUrl::App("/".into()))
        .title(constants::APP_NAME)
        .center()
        .fullscreen(false)
        .inner_size(constants::MAIN_WINDOW_WIDTH, constants::MAIN_WINDOW_HEIGHT)
        .maximizable(false)
        .maximized(false)
        .resizable(false)
        .visible(true);

    if let Err(error) = main_window.build() {
      log::error!("Error while building application: {}", error);
    } else {
      log::info!("Application build successful.");
    }
  });
  Ok(())
}
