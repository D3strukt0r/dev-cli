use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_merge::omerge;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_container: Option<String>,
    pub dumps_dir: Option<String>,
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            database_container: Some(String::from("db")),
            dumps_dir: Some(String::from("dumps"))
        }
    }
}

impl AppConfig {
    pub fn merge_from_project_root(
        project_root: impl Into<PathBuf>
    ) -> Result<Self> {
        let project_root = project_root.into();
        let mut config_file = project_root.clone();
        config_file.push(".dev-cli.yml");
        println!("config file: {:?}", &config_file);
        let mut dist_config_file = project_root.clone();
        dist_config_file.push("../../.dev-cli.dist.yml");
        println!("dist config file: {:?}", &dist_config_file);

        let config: AppConfig
            = serde_yaml::from_reader(File::open(config_file)?)?;
        println!("config: {:?}", &config);
        let dist_config: AppConfig
            = serde_yaml::from_reader(File::open(dist_config_file)?)?;
        println!("dist config: {:?}", &dist_config);
        let default_config = AppConfig::default();
        println!("default config: {:?}", &default_config);

        let merge_result  = omerge::<AppConfig, AppConfig, AppConfig>(
            default_config,
            omerge::<AppConfig, AppConfig, AppConfig>(dist_config, config)?,
        )?;

        Ok(merge_result)
    }
}
