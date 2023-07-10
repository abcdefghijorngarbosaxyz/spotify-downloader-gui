macro_rules! pub_struct {
  ($name:ident {$($field_name:ident: $field_type:ty,)*}) => {
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct $name {
      $(pub $field_name: $field_type),*
    }
  };
}

pub_struct!(AppConfig {
  always_on_top: bool,
  download_folder: String,
});

impl AppConfig {
  pub fn new() -> Self {
    Self {
      always_on_top: false,
      download_folder: tauri::api::path::download_dir()
        .unwrap()
        .to_string_lossy()
        .to_string(),
    }
  }

  pub fn get_file_path() -> std::path::PathBuf {
    crate::utils::get_app_root().join(crate::constants::APP_STORE_FILE)
  }

  pub async fn read() -> Self {
    let file_path: std::path::PathBuf = Self::get_file_path();

    if let Ok(contents) = tokio::fs::read_to_string(&file_path).await {
      if let Ok(app_config) = serde_json::from_str::<AppConfig>(&contents) {
        app_config
      } else {
        log::error!("Error parsong config file");
        Self::default()
      }
    } else {
      log::error!("Error reading config file");
      Self::default()
    }
  }

  pub async fn write(self) -> Self {
    let file_path: std::path::PathBuf = Self::get_file_path();

    if !crate::utils::path_exists(&file_path) {
      if let Err(error) = crate::utils::create_file(&file_path).await {
        log::error!("Error creating config file: {}", error);
        return self;
      }
    }

    if let Ok(contents) = serde_json::to_string_pretty(&self) {
      if let Err(error) = tokio::fs::write(&file_path, contents).await {
        log::error!("Error writing contents to config file: {}", error);
      }
    } else {
      log::error!("Error parsing contents");
    }

    self
  }

  pub fn patch(self, json: serde_json::Value) -> Self {
    let value: serde_json::Value = serde_json::to_value(&self).unwrap();
    let mut config: std::collections::HashMap<String, serde_json::Value> =
      serde_json::from_value(value).unwrap();
    let new_config: std::collections::HashMap<String, serde_json::Value> =
      serde_json::from_value(json).unwrap();

    for (k, v) in new_config {
      config.insert(k, v);
    }

    match serde_json::to_string_pretty(&config) {
      Ok(app_config) => match serde_json::from_str::<AppConfig>(&app_config) {
        Ok(app_config) => app_config,
        Err(error) => {
          log::error!("Error parsing contents: {}", error);
          self
        }
      },
      Err(error) => {
        log::error!("Error updating config: {}", error);
        self
      }
    }
  }
}

impl Default for AppConfig {
  fn default() -> Self {
    Self::new()
  }
}
