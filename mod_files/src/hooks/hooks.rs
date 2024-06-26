use fisherman::hook::builder::HookBuilder;

use crate::{ext::signature::{SignatureExt, Type}, hooks::filewrite::file_write_hook};

use super::filewrite::{virtual_to_archive_hook, VIRUTAL_TO_ARCHIVE_OG_FN};

pub struct Hooks;

impl Hooks {
    #[allow(static_mut_refs)]
    pub unsafe fn init_hooks() {
        // Get address from pattern
        let virtual_to_archive_pattern = "E8 ?? ?? ?? ?? 48 83 7B 20 08 48 8D 4B 08 72 03 48 8B 09 4C 8B 4B 18 41 B8 05 00 00 00 4D 3B C8";
        let virtual_to_archive_addr: Option<*const usize> = SignatureExt::signature(
            virtual_to_archive_pattern,
            Type::InInstruction { offset: 1, size: 5 }
        );

        // Panic if failed to get virtual to archive addr
        if virtual_to_archive_addr.is_none() {
            panic!("Couldn't create signature for virtual to archive pattern: {}", virtual_to_archive_pattern);
        } 

        // Build hook
        HookBuilder::new()
        .add_inline_hook(
            virtual_to_archive_addr.unwrap() as usize,
            virtual_to_archive_hook as usize,
            &mut VIRUTAL_TO_ARCHIVE_OG_FN,
            None
        )
        .add_iat_hook(
            "kernel32.dll",
            "CreateFileW",
            file_write_hook as usize,
        )
        .build();
    }
}