use std::{mem, ptr::null_mut};
use fisherman::{scanner::{signature::Signature, simple_scanner::SimpleScanner}, util::{get_module_slice, get_relative_pointer}};
use winapi::um::libloaderapi::GetModuleHandleA;

#[allow(unused)]
pub enum Type {
    Address,
    InInstruction { 
        offset: isize,
        size: isize,
    },
}

// Fisherman extension function(s)
pub struct SignatureExt;

impl SignatureExt {
    // Finds the address of a pattern in memory
    pub unsafe fn signature<T>(pattern: &str, r#type: Type) -> Option<*const T> {

        // Try to create signature from pattern
        match Signature::from_ida_pattern(pattern) {
            
            // Signature created
            Ok(sig) => {

                // Base address
                let base = GetModuleHandleA(null_mut()) as isize;

                // Scan process memory
                match SimpleScanner.scan(get_module_slice(base as usize), &sig) {

                    // Scan returned the offset
                    Some(offset) => {

                        // Add offset to base
                        let addr = base + offset as isize;

                        // Check if addr is in instruction or just an address
                        match r#type {

                            // Just an address
                            Type::Address => {
                                return Some(mem::transmute(get_relative_pointer::<T>(addr, 0, 0)))
                            },

                            // Addres is part of an instruction, extract based on provided offset and size
                            Type::InInstruction { offset, size } => {
                                return Some(mem::transmute(get_relative_pointer::<T>(addr, offset, size)))
                            },
                        }
                    },

                    // Failed to find pattern
                    None => {
                        println!("Could not find pattern {}", sig.signature.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "))
                    }
                }
            },
            // Failed to create signature
            Err(_) => {
                println!("Couldn't create signature for pattern: {}", pattern)
            },
        };
        None
    }
}