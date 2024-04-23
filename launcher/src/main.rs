use std::{env, process::Command, thread::sleep, time::Duration};
use config::read::config;
use steam_path::steam_path::SteamPath;
use dll_syringe::{Syringe, process::OwnedProcess};
mod steam_path;
mod config;

fn main() {
    // Locate ELDEN RING install dir
    let elden_ring_path_res= SteamPath::game_root();
    
    // Throw error if locating ELDEN RING dir failed
    if elden_ring_path_res.is_err() {
        panic!("{}",elden_ring_path_res.unwrap_err());
    }

    // Unwrap ELDEN RING dir
    let elden_ring_path = elden_ring_path_res.unwrap();

    // Launch Seamless
    Command::new(elden_ring_path.join("launch_elden_ring_seamlesscoop.exe").to_str().unwrap())
    .current_dir(elden_ring_path)
    .spawn()
    .expect("Failed to start seamlessCoop!");
    
    // Wait for ELDEN RING process
    while OwnedProcess::find_first_by_name("eldenring.exe").is_none() {}

    // Small delay before injecting dlls
    sleep(Duration::from_secs(2));

    // Use dll-syringe to inject dll
    let target_process = OwnedProcess::find_first_by_name("eldenring.exe").unwrap();
    let syringe = Syringe::for_process(target_process);

    // Inject file modding dll responsible for replacing game files with mod files if present
    syringe.inject(env::current_exe().unwrap().parent().unwrap().join("mod_files.dll")).unwrap();
    
    // Inject other dlls defined in config.toml
    unsafe {
        if let Some(dlls) = &config().dlls {
            dlls.iter().for_each(|dll| {
                syringe.inject(dll).unwrap();
            });
        } 
    }
}