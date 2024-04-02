use fisherman::{hook::builder::HookBuilder, scanner::signature::Signature};

use crate::{ext::signature::{SignatureExt, Type}, hooks::filewrite::file_write_hook};

use super::filewrite::{virtual_to_archive_hook, VIRUTAL_TO_ARCHIVE_OG_FN};

pub struct Hooks;

impl Hooks {
    #[allow(static_mut_refs)]
    pub unsafe fn init_hooks() {

        // VirtualToArchive 
        let virtual_to_archive_pattern = "E8 ?? ?? ?? ?? 48 83 7B 20 08 48 8D 4B 08 72 03 48 8B 09 4C 8B 4B 18 41 B8 05 00 00 00 4D 3B C8";
        let virtual_to_archive_sig = Signature::from_ida_pattern(virtual_to_archive_pattern);
        let virtual_to_archive_addr: *const usize = match virtual_to_archive_sig {
            Ok(sig) => {
                SignatureExt::signature(&sig, Type::InInstruction { offset: 1, size: 5 })
            },
            Err(_) => panic!("Couldn't create signature for pattern: {}", virtual_to_archive_pattern),
        };

        // Build hook
        HookBuilder::new()
        .add_inline_hook(
            virtual_to_archive_addr as usize,
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