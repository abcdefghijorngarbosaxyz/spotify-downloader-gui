pub fn get_app_root() -> std::path::PathBuf {
  tauri::api::path::home_dir().unwrap().join(".spotdl")
}

pub fn path_exists(path: &std::path::Path) -> bool {
  std::path::Path::new(path).exists()
}

pub async fn create_file<P: AsRef<std::path::Path>>(file_path: P) -> anyhow::Result<()> {
  let file_path: &std::path::Path = file_path.as_ref();
  if let Some(parent) = file_path.parent() {
    if !parent.exists() {
      std::fs::create_dir_all(parent)?;
    }
  }
  std::fs::File::create(file_path)?;
  anyhow::Ok(())
}
