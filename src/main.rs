mod app;
mod constants;

#[tokio::main]
async fn main() {
  let context: tauri::Context<tauri::utils::assets::EmbeddedAssets> = tauri::generate_context!();

  let app_logger: tauri_plugin_log::Builder = tauri_plugin_log::Builder::default().targets([
    tauri_plugin_log::LogTarget::LogDir,
    tauri_plugin_log::LogTarget::Stdout,
    tauri_plugin_log::LogTarget::Stderr,
    tauri_plugin_log::LogTarget::Webview,
  ]);

  let app_builder = tauri::Builder::default()
    .plugin(app_logger.build())
    .setup(app::setup::init);

  if let Err(error) = app_builder.run(context) {
    log::error!("Error while running generating context: {}", error);
  } else {
    log::info!("Context generated.");
  }
}
