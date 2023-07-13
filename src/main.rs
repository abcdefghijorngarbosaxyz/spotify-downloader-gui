#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod app;
mod config;
mod constants;
mod utils;

use app::{generic, window};
use config::AppConfig;

#[tokio::main]
async fn main() {
  let context: tauri::Context<tauri::utils::assets::EmbeddedAssets> = tauri::generate_context!();

  let _ = AppConfig::read().await.write().await;

  let app_logger: tauri_plugin_log::Builder = tauri_plugin_log::Builder::default().targets([
    tauri_plugin_log::LogTarget::LogDir,
    tauri_plugin_log::LogTarget::Stdout,
    tauri_plugin_log::LogTarget::Stderr,
    tauri_plugin_log::LogTarget::Webview,
  ]);

  let app_builder = tauri::Builder::default()
    .plugin(app_logger.build())
    .invoke_handler(tauri::generate_handler![
      generic::cmd::get_platform,
      generic::cmd::join_us_on_discord,
      generic::cmd::report_issue,
      generic::cmd::docs,
      generic::cmd::devtools,
      generic::cmd::minimize,
      generic::cmd::always_on_top,
      generic::cmd::about_window,
      window::cmd::close_window,
    ])
    .setup(app::setup::init);

  /*
   * Some of the menu items are configured also for windows, but
   * we disabled it to allow full customization on the titlebar(on windows)
   * and Tauri, currently, can't change the theme of the native menubar.
   */
  #[cfg(target_os = "macos")]
  {
    app_builder = app_builder.menu(app::menu::init());
  }

  #[cfg(target_os = "macos")]
  {
    app_builder = app_builder.on_menu_event(|event| {
      tokio::task::spawn(async move { app::menu::handle_event(event).await });
    });
  }

  app_builder
    .on_window_event(app::window::handle_event)
    .run(context)
    .expect("Error while running application");
}
