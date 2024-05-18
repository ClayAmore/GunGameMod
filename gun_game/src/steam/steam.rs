use super::callbacks::{callback::SteamCallback, lobby_created_cb::LOBBY_CREATED_VFTABLE};

#[derive(Default)]
pub struct Steam;

impl Steam {
    // Register steam callbacks
    pub fn register_callbacks() {
        let cb = Box::leak(Box::new(
            SteamCallback::new(&LOBBY_CREATED_VFTABLE)
        ));

        unsafe { steamworks_sys::SteamAPI_RegisterCallback(
            cb as *mut _ as *mut _,
            steamworks_sys::LobbyCreated_t_k_iCallback as _,
        ) }
    }
}
