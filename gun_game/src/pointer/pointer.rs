
pub struct Pointer;

impl Pointer {
    pub unsafe fn from_offsets<T>(base: *const usize, offsets: &[isize]) -> Option<*const T> {
        let mut addr = base as *const usize;
        for offset in offsets {
            if *addr as *const usize == std::ptr::null() { return None; }
            addr = (*addr as *const usize).byte_offset(*offset);
        }
        Some(addr as *const T)
    }
}