#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod constants;
mod utils;

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
    .setup(app::setup::init)
    .menu(app::menu::init());

  app_builder
    .on_menu_event(|event| {
      tokio::task::spawn(async move { app::menu::handle_event(event).await });
    })
    .on_window_event(app::window::handle_event)
    .run(context)
    .expect("Error while running application");
}
