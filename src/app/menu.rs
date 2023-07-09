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
        tauri::CustomMenuItem::new("devtools", "Toggle Dev Tools")
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

pub fn handle_event(event: tauri::WindowMenuEvent<tauri::Wry>) {
  let window: &tauri::Window = Some(event.window()).unwrap();
  let app_handle: tauri::AppHandle = window.app_handle();
  let menu_id: &str = event.menu_item_id();
  let menu_handle: tauri::window::MenuHandle = window.menu_handle();

  match menu_id {
    "docs" => open(&app_handle, crate::constants::DOCS_URL),
    "report_issue" => open(&app_handle, crate::constants::ISSUES_URL),
    "join_us_on_discord" => open(&app_handle, crate::constants::DISCORD_URL),
    _ => {}
  }
}

fn open(app_handle: &tauri::AppHandle, path: &str) {
  tauri::api::shell::open(&app_handle.shell_scope(), path, None).unwrap();
}
