use crate::{signature::signature::{SignatureExt, Type}, pointer::pointer::Pointer};

pub struct GameDataMan;

// For testing
#[allow(unused)]
impl GameDataMan {
    pub unsafe fn set_player_weapon(weapon_id: u32) {
        // Get address from pattern
        let game_data_man = SignatureExt::signature(
            "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 05 48 8B 40 58 C3 C3", 
            Type::InInstruction { offset: 3, size: 7 }
        );

        if game_data_man.is_none() {
            panic!("Failed!");
        }

        let result = Pointer::from_offsets_mut::<u32>(game_data_man.unwrap(), &[0x8, 0x3A0]);
        
        if let Some(left_hand_weapon_1_ptr) = result {
            *left_hand_weapon_1_ptr = weapon_id;
        }
    }
}