use crate::{
    Result,
    mmap::{MapFlags, Mmap, ProtFlags},
    object::ElfObject,
    segment::PAGE_SIZE,
};
use alloc::alloc::{dealloc, handle_alloc_error};
use core::{alloc::Layout, ptr::NonNull, slice::from_raw_parts_mut};

/// An implementation of Mmap trait
pub struct MmapImpl;

impl Mmap for MmapImpl {
    unsafe fn mmap(
        addr: Option<usize>,
        len: usize,
        _prot: ProtFlags,
        flags: MapFlags,
        _offset: usize,
        _fd: Option<isize>,
        need_copy: &mut bool,
    ) -> crate::Result<core::ptr::NonNull<core::ffi::c_void>> {
        *need_copy = true;
        if let Some(addr) = addr {
            let ptr = addr as *mut u8;
            Ok(unsafe { NonNull::new_unchecked(ptr as _) })
        } else {
            // 只有创建整个空间时会走这条路径
            assert!((MapFlags::MAP_FIXED & flags).bits() == 0);
            let layout = unsafe { Layout::from_size_align_unchecked(len, PAGE_SIZE) };
            let memory = unsafe { alloc::alloc::alloc(layout) };
            if memory.is_null() {
                handle_alloc_error(layout);
            }
            // use this set prot to test no_mmap
            //libc::mprotect(memory as _, len, crate::mmap::ProtFlags::all().bits());
            Ok(unsafe { NonNull::new_unchecked(memory as _) })
        }
    }

    unsafe fn mmap_anonymous(
        addr: usize,
        len: usize,
        _prot: ProtFlags,
        _flags: MapFlags,
    ) -> crate::Result<core::ptr::NonNull<core::ffi::c_void>> {
        let ptr = addr as *mut u8;
        let dest = unsafe { from_raw_parts_mut(ptr, len) };
        dest.fill(0);
        Ok(unsafe { NonNull::new_unchecked(ptr as _) })
    }

    unsafe fn munmap(addr: core::ptr::NonNull<core::ffi::c_void>, len: usize) -> crate::Result<()> {
        unsafe {
            dealloc(
                addr.as_ptr() as _,
                Layout::from_size_align_unchecked(len, PAGE_SIZE),
            )
        };
        Ok(())
    }

    unsafe fn mprotect(
        _addr: NonNull<core::ffi::c_void>,
        _len: usize,
        _prot: ProtFlags,
    ) -> crate::Result<()> {
        Ok(())
    }
}

pub(crate) struct RawFile;

impl RawFile {
    pub(crate) fn from_path(_path: &str) -> Result<Self> {
        unimplemented!()
    }

    pub(crate) fn from_owned_fd(_path: &str, _raw_fd: i32) -> Self {
        todo!()
    }
}

impl ElfObject for RawFile {
    fn file_name(&self) -> &core::ffi::CStr {
        todo!()
    }

    fn read(&mut self, _buf: &mut [u8], _offset: usize) -> Result<()> {
        todo!()
    }

    fn as_fd(&self) -> Option<isize> {
        todo!()
    }
}
