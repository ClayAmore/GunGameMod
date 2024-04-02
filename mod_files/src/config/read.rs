use std::{collections::HashMap, fs, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use toml::de::Error;
use walkdir::WalkDir;

use crate::utils::Utils;

#[derive(Deserialize)]
pub struct Config {
    mods: Option<Vec<String>>,
}

pub unsafe fn config() -> &'static Config {
    static CONFIG: OnceCell<Config> = OnceCell::new();
    CONFIG.get_or_init(|| {
        let config_path = Path::new(Utils::dll_path()).join("config.toml");
        let config = fs::read_to_string(config_path.to_str().unwrap().to_string()).expect("Failed to read config.toml");
        let config_without_comments: Vec<&str> = config.lines().filter(|line| !line.starts_with("#")).collect();
        let config_res: Result<Config, Error> = toml::from_str(&config_without_comments.join("\n"));

        match config_res {
            Ok(config) => {
                let mods: Option<Vec<String>> = match config.mods {
                    Some(mods) => {
                        let mods = mods.into_iter()
                            .filter(|mod_path_str| !mod_path_str.is_empty())
                            .map(|mod_path_str| {
                            let mod_path = Path::new(&mod_path_str);
                            if mod_path.is_relative() {
                                Path::new(Utils::dll_path()).join(mod_path).to_str().unwrap().to_string()
                            }
                            else {
                                mod_path_str
                            }
                        }).collect::<Vec<String>>();

                        Some(mods)
                    },
                    None => None,
                };

                Config{
                    mods
                }
            },
            Err(err) => {
                panic!("Failed to parse config.toml\n{err}");
            },
        }
    })
}

pub unsafe fn mod_files() -> &'static HashMap<String, String> {
    static MOD_FILES: OnceCell<HashMap<String, String>> = OnceCell::new();
    MOD_FILES.get_or_init(|| {
        let mut map: HashMap<String, String> = HashMap::new();
        if let Some(mods) = &config().mods {
            mods.iter().for_each(|mod_path| {
                WalkDir::new(mod_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
                .for_each(|e| {
                    let path = String::from(e.path().to_string_lossy());
                    let name_os_str = Path::new(&path).file_name();
                    if name_os_str.is_none() { panic!("Failed to get file_name. Couldn't get from path");}
                    let name_str = name_os_str.unwrap().to_str();
                    if name_str.is_none() { panic!("Failed to get file_name. Couldn't convert osstr to string");}
                    map.insert(name_str.unwrap().to_string(), path.to_string());
                });
            });
        }
        map
    })
}
