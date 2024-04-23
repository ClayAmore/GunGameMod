use std::thread::{self, sleep};
use std::time::Duration;

use hooks::hooks::Hooks;
use winapi::um::consoleapi::AllocConsole;
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use steamworks::{Client, LobbyDataUpdate, P2PSessionRequest};
use std::mem::ManuallyDrop;

use crate::ext::steamworks_ext::lobbycreated_t::LobbyCreated;

// Extensions for external crates
mod ext;

// Pointer
mod pointer;

// Hooks
mod hooks;

// Inventory Management
mod inventory;

// PlayerIns
mod player;

// Utils
mod utils;

// Main thread
unsafe fn main_thread() {
    #[cfg(debug_assertions)]
    unsafe {AllocConsole();}

    // init hooks
    Hooks::init_hooks();

    // Steam testing
    let manually_drop = ManuallyDrop::new(Client::init().unwrap());
    let client = &manually_drop.0;
    let single = &manually_drop.1;
    
    let user = client.user();
    let matchmaking = client.matchmaking();
    let networking = client.networking();

    println!("{}", user.steam_id().raw()); // <--- works 

    // Custom callback /ext/steamworks_ext/lobbycreated_t
    client.register_callback(|_: LobbyCreated| { 
        println!("Lobby Enter Update");
    });

    // Steamworks callback
    client.register_callback(|_: LobbyDataUpdate| {
        println!("Lobby Data Update");
    });

    // Steamworks callback
    client.register_callback(|_: P2PSessionRequest| {
        println!("P2PSessionRequest");
    });

    loop {
        single.run_callbacks();
        sleep(Duration::from_millis(100));
    }
}

// Dll entry
#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: LPVOID) -> BOOL {
if dw_reason == DLL_PROCESS_ATTACH {
    unsafe {
        DisableThreadLibraryCalls(h_module);
    }
    thread::spawn(|| {
        unsafe { main_thread() };
    });
}
TRUE
}