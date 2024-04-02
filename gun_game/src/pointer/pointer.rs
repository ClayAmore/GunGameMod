
pub struct Pointer;

impl Pointer {

    // Pointer traversal
    pub unsafe fn from_offsets<T>(base: *const usize, offsets: &[isize]) -> Option<*const T> {
        // Start from base address
        let mut addr = base as *const usize;

        // Loop offsets, deref and add offset
        for offset in offsets {
            
            // If deref addr is null return None 
            if *addr as *const usize == std::ptr::null() { return None; }

            // Add offset to deref addr
            addr = (*addr as *const usize).byte_offset(*offset);
        }

        // Return addr as pointer to user defined type T
        Some(addr as *const T)
    }
}