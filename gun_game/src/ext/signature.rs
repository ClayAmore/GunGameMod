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

pub struct SignatureExt;

impl SignatureExt {
    pub unsafe fn signature<T>(pattern: &str, r#type: Type) -> Option<*const T> {
        match Signature::from_ida_pattern(pattern) {
            Ok(sig) => {
                let base = GetModuleHandleA(null_mut()) as isize;
                match SimpleScanner.scan(get_module_slice(base as usize), &sig) {
                    None => {
                        println!("Could not find pattern {}", sig.signature.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "))
                    },
                    Some(offset) => {
                        let addr = base + offset as isize;
                        match r#type {
                            Type::Address => {
                                return Some(mem::transmute(get_relative_pointer::<T>(addr, 0, 0)))
                            },
                            Type::InInstruction { offset, size } => {
                                return Some(mem::transmute(get_relative_pointer::<T>(addr, offset, size)))
                            },
                        }
                    }
                }
            },
            Err(_) => {
                println!("Couldn't create signature for pattern: {}", pattern)
            },
        };
        None
    }
}