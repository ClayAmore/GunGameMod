use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use keyvalues_parser::Obj;
use keyvalues_parser::Value;
use winreg::enums::*;
use winreg::RegKey;

pub struct SteamPath;
use keyvalues_parser::Vdf;

const LIBRARY_FOLDERS: &str = "libraryfolders.vdf";

impl SteamPath {
    pub fn game_root() -> Result<PathBuf, Error> {

        // Lookg up steam path in the registery
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let steam_reg = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;
        let steam_path_str: String = steam_reg.get_value("SteamPath")?;

        // Create path to SteamApps
        let steam_app_path = Path::new(&steam_path_str).join("SteamApps");

        // Check if folder exists, fail if not
        if !steam_app_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Couldn't find `SteamApps` in {}", steam_path_str)));
        } 

        // Parse library folders vdf
        let library_folders_path = steam_app_path.join(LIBRARY_FOLDERS);
        let library_folders_vdf_data = fs::read_to_string(library_folders_path)?;
        let library_folder_vdf_result = Vdf::parse(&library_folders_vdf_data);

        // If parsing failed, throw error
        if library_folder_vdf_result.is_err() {
            return Err(Error::new(ErrorKind::InvalidData, format!("{:#?}", library_folder_vdf_result.err())));
        }

        // Unwrap if no error
        let library_folder_vdf = library_folder_vdf_result.unwrap();
        
        // Check if vdf can be turned to obj
        if library_folder_vdf.value.get_obj().is_none() {
            return Err(Error::new(ErrorKind::NotFound, format!("Cannot locate game. LibraryFolder.vdf appears to be empty!")));
        }

        // Create path vec from library folders
        let mut library_folder_paths: Vec<String> = Vec::new();
        let library_folder_vdf_obj = library_folder_vdf.value.unwrap_obj();
        Self::fill_paths(&library_folder_vdf_obj, &mut library_folder_paths);

        // Loop all steam game directories looking for ELDEN RING
        let mut elden_ring_dir = PathBuf::new();
        for path_str in library_folder_paths {
            let path = Path::new(&path_str);

            // Throw error if one of the steam game paths doesn't exists
            if !path.exists() {
                return Err(Error::new(ErrorKind::NotFound, format!("Steam game directory doesn't exist!")));
            }

            elden_ring_dir = path.join("SteamApps").join("common").join("ELDEN RING").join("Game");

            // Break if elden ring path is found
            if elden_ring_dir.exists() {
                break;
            }
        }

        // Throw error if elden ring dir was not found
        if !elden_ring_dir.exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Couldn't locate ELDEN RING directory in any of the steam game paths!")));
        }
        
        Ok(elden_ring_dir)
    }

    // Stores paths from LibraryFolder.vdf, loops recursively
    fn fill_paths<'a>(obj: &Obj, paths: &mut Vec<String>) {
        for (key, value) in obj.iter() {
            if key == "path" {
                match &value[0] {
                    Value::Str(path) => {paths.push(path.to_string());},
                    Value::Obj(_) => panic!("Path Shouldn't be an object!"),
                }
            }
            else {
                for inner_value in value {
                    match inner_value {
                        Value::Str(_) => {
                            // Ignore other strings
                        },
                        Value::Obj(obj) => {
                            Self::fill_paths(obj, paths);
                        },
                    }
                }
            }
        }
    }
}