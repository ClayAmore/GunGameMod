use super::callback::{create_vtable, SteamCallbackImpl, SteamCallbackVftable};

pub struct LobbyCreatedCallback;

impl SteamCallbackImpl for LobbyCreatedCallback {
    type TData = steamworks_sys::LobbyCreated_t;

    fn run(data: *const Self::TData) {
        let steam_lobby_id = unsafe { *data }.m_ulSteamIDLobby;
        println!("LobbyID: {steam_lobby_id}");
    }
}

pub static LOBBY_CREATED_VFTABLE: SteamCallbackVftable<LobbyCreatedCallback> =
    create_vtable();

