use crate::{signature::signature::{SignatureExt, Type}, pointer::pointer::Pointer};

pub struct Player;

// For testing
#[allow(unused)]
impl Player {
    pub unsafe fn is_player_loaded() -> bool {
        // Get address from pattern
        let world_chr_man_addr = SignatureExt::signature(
            "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 0F 48 39 88", 
            Type::InInstruction { offset: 3, size: 7 }
        );

        // Check if playerins exists if result is not None
        if world_chr_man_addr.is_some() {
            return Pointer::from_offsets::<*const usize>(world_chr_man_addr.unwrap(), &[0x10EF8, 0x10]).is_some();
        } 

        // Default to false
        false
    }
}