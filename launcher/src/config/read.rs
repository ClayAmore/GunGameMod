use std::{env, fs, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use toml::de::Error;

#[derive(Deserialize)]
pub struct Config {
    pub dlls: Option<Vec<String>>,
}

pub unsafe fn config() -> &'static Config {
    static CONFIG: OnceCell<Config> = OnceCell::new();
    CONFIG.get_or_init(|| {
        let curren_dir = env::current_dir().unwrap();
        let config_path = curren_dir.join("config.toml");
        let config = fs::read_to_string(config_path.to_str().unwrap().to_string()).expect("Failed to read config.toml");
        let config_without_comments: Vec<&str> = config.lines().filter(|line| !line.starts_with("#")).collect();
        let config_res: Result<Config, Error> = toml::from_str(&config_without_comments.join("\n"));

        match config_res {
            Ok(config) => {
                let dlls: Option<Vec<String>> = match config.dlls {
                    Some(dlls) => {
                        let dlls = dlls.into_iter()
                        .filter(|dll_path_str| !dll_path_str.is_empty())
                        .map(|dll_path_str| {
                            let mod_path = Path::new(&dll_path_str);

                            if mod_path.is_relative() {
                                curren_dir.join(mod_path).to_str().unwrap().to_string()
                            }
                            else {
                                dll_path_str
                            }
                        }).collect::<Vec<String>>();

                        Some(dlls)
                    },
                    None => None,
                };
                Config{
                    dlls
                }
            },
            Err(err) => {
                panic!("Failed to parse config.toml\n{err}");
            },
        }
    })
}