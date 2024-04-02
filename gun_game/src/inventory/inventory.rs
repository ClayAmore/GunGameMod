
use crate::{ext::signature::{SignatureExt, Type}, pointer::pointer::Pointer};

// Equip Inventory Data [[[GameDataMan]+0x8]+0x2B0+0x158+0x40]
// Weapon Match Making level [[GameDataMan]+0x8]+E2

pub struct Inventory;

impl Inventory {
    pub unsafe fn get_weapon_matchmaking_level() -> Option<*const u8> {
        // Get address from pattern
        let game_data_man_addr = SignatureExt::signature(
            "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 05 48 8B 40 58 C3 C3",
            Type::InInstruction { offset: 3, size: 7 }
        );

        // Try get addr from offsets if signature result is not none
        if game_data_man_addr.is_some() {
            return Pointer::from_offsets(game_data_man_addr.unwrap(), &[0x8, 0xE2]);
        } 

        // Default to none
        None
    }
}

