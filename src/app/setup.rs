use tauri::Manager;

pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
  let app_handle: tauri::AppHandle = app.app_handle();

  tokio::task::spawn(async move {
    let app_config = crate::config::AppConfig::read().await;

    let mut main_window: tauri::WindowBuilder<'_> =
      tauri::WindowBuilder::new(&app_handle, "main", tauri::WindowUrl::App("/".into()))
        .title(crate::constants::APP_NAME)
        .center()
        .fullscreen(false)
        .inner_size(
          crate::constants::DEFAULT_WINDOW_WIDTH,
          crate::constants::DEFAULT_WINDOW_HEIGHT,
        )
        .maximizable(false)
        .maximized(false)
        .resizable(false)
        .always_on_top(app_config.always_on_top)
        .visible(true);

    /*
     * Disable decorations on windows for custom titlebar
     */
    #[cfg(target_os = "windows")]
    {
      main_window = main_window.decorations(false);
    }

    if let Err(error) = main_window.build() {
      log::error!("Error while building application: {}", error);
    } else {
      log::info!("Application build successful.");
    }
  });
  Ok(())
}
