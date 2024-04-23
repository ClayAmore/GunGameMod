use std::ffi::c_void;
use steamworks::{LobbyId, Callback};
use steamworks_sys as sys;

/// A lobby chat room state has changed, this is usually upon attempting to enter a lobby.
#[derive(Clone, Debug)]
#[allow(unused)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LobbyCreated {
    /// The Steam ID of the lobby.
    pub lobby: LobbyId,
    /// The result of the operation.
    pub result: sys::EResult,
}

unsafe impl Callback for LobbyCreated {
    const ID: i32 = 513;
    const SIZE: i32 = ::std::mem::size_of::<sys::LobbyCreated_t>() as i32;

    unsafe fn from_raw(raw: *mut c_void) -> Self {
        let val = &mut *(raw as *mut sys::LobbyCreated_t);

        LobbyCreated {
            lobby: LobbyId::from_raw(val.m_ulSteamIDLobby),
            result: val.m_eResult
        }
    }
}