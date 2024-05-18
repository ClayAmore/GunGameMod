// Credits: ChainFailure (https://github.com/vswarte) 
// For figuring out how to implement callbacks

use std::os::raw;

use steamworks_sys::uint8;

#[repr(C)]
pub struct SteamCallbackVftable<C: SteamCallbackImpl> {
    pub run: fn(usize, *const C::TData),
    pub run_other: fn(usize, *const C::TData, bool, u64),
    pub get_callback_size_bytes: fn(usize) -> u32,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct SteamCallback<C: SteamCallbackImpl> {
    vtable: *const SteamCallbackVftable<C>,
    m_nCallbackFlags: uint8,
    m_iCallback: raw::c_int,
}

impl<C: SteamCallbackImpl> SteamCallback<C> {
    pub fn new(
        vtable: *const SteamCallbackVftable<C>
    ) -> Self {
        Self {
            vtable,
            m_nCallbackFlags: 0,
            m_iCallback: 0,
        }
    }
}

pub trait SteamCallbackImpl {
    type TData: Sized;
    fn run(data: *const Self::TData);
}

pub const fn create_vtable<C: SteamCallbackImpl>() -> SteamCallbackVftable<C> {
    SteamCallbackVftable {
        run: |_, data| C::run(data),
        run_other: |_, data, _, _| C::run(data),
        get_callback_size_bytes: |_| {
            std::mem::size_of::<C::TData>() as u32
        },
    }
}