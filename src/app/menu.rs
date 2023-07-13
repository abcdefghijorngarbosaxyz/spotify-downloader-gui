use tauri::Manager;

pub fn init() -> tauri::Menu {
  let app_menu: tauri::Menu = tauri::Menu::with_items([
    #[cfg(target_os = "macos")]
    tauri::MenuEntry::Submenu(tauri::Submenu::new(
      name,
      tauri::Menu::with_items([
        tauri::MenuItem::About(name, tauri::AboutMetadata::default()).into(),
        tauri::MenuItem::Separator.into(),
        tauri::MenuItem::Services.into(),
        tauri::MenuItem::Separator.into(),
        tauri::MenuItem::Hide.into(),
        tauri::MenuItem::HideOthers.into(),
        tauri::MenuItem::ShowAll.into(),
        tauri::MenuItem::Separator.into(),
        tauri::MenuItem::Quit.into(),
      ]),
    )),
    tauri::MenuEntry::Submenu(tauri::Submenu::new(
      "File",
      tauri::Menu::with_items([
        tauri::CustomMenuItem::new("open_download_folder", "Open Download Folder")
          .accelerator("CmdOrCtrl+O")
          .into(),
        tauri::CustomMenuItem::new("select_download_folder", "Select Download Folder")
          .accelerator("CmdOrCtrl+Shift+O")
          .into(),
        tauri::MenuItem::Separator.into(),
        #[cfg(not(target_os = "macos"))]
        tauri::CustomMenuItem::new("options", "Options...")
          .accelerator("CmdOrCtrl+,")
          .into(),
        #[cfg(not(target_os = "macos"))]
        tauri::MenuItem::Separator.into(),
        #[cfg(not(target_os = "macos"))]
        tauri::MenuItem::CloseWindow.into(),
      ]),
    )),
    tauri::MenuEntry::Submenu(tauri::Submenu::new(
      "Edit",
      tauri::Menu::with_items([
        #[cfg(target_os = "macos")]
        tauri::MenuItem::Undo.into(),
        #[cfg(target_os = "macos")]
        tauri::MenuItem::Redo.into(),
        #[cfg(target_os = "macos")]
        tauri::MenuItem::Separator.into(),
        tauri::MenuItem::Cut.into(),
        tauri::MenuItem::Copy.into(),
        tauri::MenuItem::Paste.into(),
      ]),
    )),
    tauri::MenuEntry::Submenu(tauri::Submenu::new(
      "View",
      tauri::Menu::with_items([
        tauri::CustomMenuItem::new("downloads", "Downloads")
          .accelerator("CmdOrCtrl+K")
          .into(),
        tauri::CustomMenuItem::new("show_status_bar", "Show Status Bar")
          .accelerator("CmdOrCtrl+J")
          .into(),
        #[cfg(target_os = "macos")]
        tauri::MenuItem::Separator.into(),
        #[cfg(target_os = "macos")]
        tauri::MenuItem::Zoom.into(),
        #[cfg(target_os = "macos")]
        tauri::MenuItem::Separator.into(),
        #[cfg(target_os = "macos")]
        tauri::MenuItem::EnterFullScreen.into(),
        tauri::MenuItem::Separator.into(),
        tauri::CustomMenuItem::new("devtools", "Open Dev Tools")
          .accelerator("CmdOrCtrl+Shift+I")
          .into(),
      ]),
    )),
    tauri::MenuEntry::Submenu(tauri::Submenu::new(
      "Window",
      tauri::Menu::with_items([
        tauri::MenuItem::Minimize.into(),
        tauri::MenuItem::Separator.into(),
        tauri::CustomMenuItem::new("always_on_top", "Always on Top")
          .accelerator("CmdOrCtrl+T")
          .into(),
      ]),
    )),
    tauri::MenuEntry::Submenu(tauri::Submenu::new(
      "Help",
      tauri::Menu::with_items([
        tauri::CustomMenuItem::new("docs", "Documentation").into(),
        tauri::CustomMenuItem::new("show_release_notes", "Show Release Notes").into(),
        tauri::MenuItem::Separator.into(),
        tauri::CustomMenuItem::new("report_issue", "Report Issue").into(),
        tauri::CustomMenuItem::new("join_us_on_discord", "Join Us on Discord").into(),
        tauri::MenuItem::Separator.into(),
        tauri::CustomMenuItem::new("check_for_updates", "Check for Updates...").into(),
        #[cfg(not(target_os = "macos"))]
        tauri::MenuItem::Separator.into(),
        #[cfg(not(target_os = "macos"))]
        tauri::CustomMenuItem::new("about", "About").into(),
      ]),
    )),
  ]);

  app_menu
}

pub async fn handle_event(event: tauri::WindowMenuEvent<tauri::Wry>) {
  let window: &tauri::Window = Some(event.window()).unwrap();
  let app_handle: tauri::AppHandle = window.app_handle();
  let menu_id: &str = event.menu_item_id();
  let menu_handle: tauri::window::MenuHandle = window.menu_handle();

  let app_config: crate::config::AppConfig = crate::config::AppConfig::read().await;

  match menu_id {
    "docs" => open(&app_handle, crate::constants::DOCS_URL),
    "report_issue" => open(&app_handle, crate::constants::ISSUES_URL),
    "join_us_on_discord" => open(&app_handle, crate::constants::DISCORD_URL),
    "devtools" => {
      if window.is_devtools_open() {
        window.close_devtools();
      } else {
        window.open_devtools();
      }
    }
    "about" => crate::app::about::open_about(app_handle, window.clone()),
    "always_on_top" => {
      let always_on_top: bool = !app_config.always_on_top;

      menu_handle
        .get_item(menu_id)
        .set_selected(always_on_top)
        .unwrap();
      window.set_always_on_top(always_on_top).unwrap();
      app_config
        .patch(serde_json::json!({ "always_on_top": always_on_top }))
        .write()
        .await;
    }
    "open_download_folder" => {
      if crate::utils::path_exists(std::path::PathBuf::from(&app_config.download_folder).as_path())
      {
        open(&app_handle, &app_config.download_folder);
      } else {
        tauri::api::dialog::MessageDialogBuilder::new("Folder Not Found", "Select another folder.")
          .buttons(tauri::api::dialog::MessageDialogButtons::Ok)
          .parent(&window)
          .kind(tauri::api::dialog::MessageDialogKind::Error)
          .show(|_| {});
      }
    }

    "select_download_folder" => tokio::task::spawn(async move {
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
    .unwrap(),
    _ => {}
  }
}

fn open(app_handle: &tauri::AppHandle, path: &str) {
  tauri::api::shell::open(&app_handle.shell_scope(), path, None).unwrap();
}
