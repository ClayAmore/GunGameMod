use std::{env, fs, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use toml::de::Error;

#[derive(Deserialize)]
pub struct Config {
    pub dlls: Option<Vec<String>>,
}

// Reads the config.toml once if called and stores the dlls as 
// static list in case of future calls
pub unsafe fn config() -> &'static Config {
    static CONFIG: OnceCell<Config> = OnceCell::new();
    CONFIG.get_or_init(|| {
        // Current dir
        let curren_dir = env::current_dir().unwrap();

        // Config file path
        let config_path = curren_dir.join("config.toml");

        // Read config file
        let config = fs::read_to_string(config_path.to_str().unwrap().to_string()).expect("Failed to read config.toml");

        // Exclude commented lines
        let config_without_comments: Vec<&str> = config.lines().filter(|line| !line.starts_with("#")).collect();

        // Join lines into one string
        let config_res: Result<Config, Error> = toml::from_str(&config_without_comments.join("\n"));

        // Parse config.toml and create a dll path list
        match config_res {

            // Parse was a success
            Ok(config) => {

                // Check if dlls was present in the config.toml. Won't be if commented out
                let dlls: Option<Vec<String>> = match config.dlls {
                    
                    // Dlls present
                    Some(dlls) => {

                        // Iterate dlls, filter out empty paths and convert relative paths to absolute
                        let dlls = dlls.into_iter()
                        .filter(|dll_path_str| !dll_path_str.is_empty())
                        .map(|dll_path_str| {
                            let mod_path = Path::new(&dll_path_str);

                            // Relative path to absolute
                            if mod_path.is_relative() {
                                curren_dir.join(mod_path).to_str().unwrap().to_string()
                            }

                            // Absolute path stays as is
                            else {
                                dll_path_str
                            }
                        }).collect::<Vec<String>>();

                        Some(dlls)
                    },

                    // Dlls not present, return value None
                    None => None,
                };
                Config{
                    dlls
                }
            },
            // Parse failed
            Err(err) => {
                panic!("Failed to parse config.toml\n{err}");
            },
        }
    })
}