use std::{collections::HashMap, fs, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use toml::de::Error;
use walkdir::WalkDir;

use crate::utils::Utils;

// Struct for config file for use with serde Deserialization
#[derive(Deserialize)]
pub struct Config {
    mods: Option<Vec<String>>,
}

// Reads the config.toml once if called and stores the mod directories as 
// static list in case of future calls
pub unsafe fn config() -> &'static Config {
    static CONFIG: OnceCell<Config> = OnceCell::new();
    CONFIG.get_or_init(|| {
        // Config paths from dll_path, assumes config is at same dir as dll
        let config_path = Path::new(Utils::dll_path()).join("config.toml");

        // Read config file
        let config = fs::read_to_string(config_path.to_str().unwrap().to_string()).expect("Failed to read config.toml");

        // Exclude commented lines
        let config_without_comments: Vec<&str> = config.lines().filter(|line| !line.starts_with("#")).collect();

        // Join lines into one string
        let config_res: Result<Config, Error> = toml::from_str(&config_without_comments.join("\n"));

        // Parse config.toml and create a mod directories list
        match config_res {

            // Parse was a success
            Ok(config) => {
                
                // Check if mods were present in the config.toml. Won't be if commented out
                let mods: Option<Vec<String>> = match config.mods {
                    
                    // Mods present
                    Some(mods) => {
                        
                        // Iterate mod directory paths, filter out empty paths and convert relative paths to absolute
                        let mods = mods.into_iter()
                            .filter(|mod_path_str| !mod_path_str.is_empty())
                            .map(|mod_path_str| {
                            let mod_path = Path::new(&mod_path_str);
                            
                            // Relative path to absolute
                            if mod_path.is_relative() {
                                Path::new(Utils::dll_path()).join(mod_path).to_str().unwrap().to_string()
                            }
                            
                            // Absolute path stays as is
                            else {
                                mod_path_str
                            }
                        }).collect::<Vec<String>>();

                        Some(mods)
                    },
                    
                    // Mods not present, return value None
                    None => None,
                };

                Config{
                    mods
                }
            },

            // Parse failed
            Err(err) => {
                panic!("Failed to parse config.toml\n{err}");
            },
        }
    })
}

// Create a Hashmap from all mod files in all mod directories. Will overwrite duplicate files with the latest.
// Example: if mod1 and mod2 both edit Roundtable map, then Roundtable map will show mod2 version. 
pub unsafe fn mod_files() -> &'static HashMap<String, String> {

    // Static Hashmap with OnceCell
    static MOD_FILES: OnceCell<HashMap<String, String>> = OnceCell::new();

    // Initiate the mod hashmap if first time otherwise just get
    MOD_FILES.get_or_init(|| {

        // New hashmap to store the mod file paths with mod file names as keys
        let mut map: HashMap<String, String> = HashMap::new();

        // Iterate mods directories
        if let Some(mods) = &config().mods {
            mods.iter().for_each(|mod_path| {

                // Iterate all mod files in a mod dir recursively using WalkPath,
                // Filter out directories
                WalkDir::new(mod_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
                .for_each(|e| {
                    // Store mod file path
                    let path = String::from(e.path().to_string_lossy());

                    // Extract name from mod file path
                    let name_os_str = Path::new(&path).file_name();
                    if name_os_str.is_none() { panic!("Failed to get file_name. Couldn't get from path");}
                    let name_str = name_os_str.unwrap().to_str();
                    if name_str.is_none() { panic!("Failed to get file_name. Couldn't convert osstr to string");}

                    // Insert mod file path into hashmap with mod file name as key
                    map.insert(name_str.unwrap().to_string(), path.to_string());
                });
            });
        }

        // return hashmap
        map
    })
}
