#![no_std]

#[macro_use]
extern crate bitflags;

pub mod flags;
pub use flags::StivaleFlags;

pub mod framebuffer;
pub use framebuffer::FramebufferInfo;

pub unsafe fn load(address: usize) -> StivaleStructure {
    let inner = &*(address as *const StivaleStructureInner);
    StivaleStructure { inner }
}

pub struct StivaleStructure {
    inner: *const StivaleStructureInner,
}

#[repr(packed)]
struct StivaleStructureInner {
    cmdline: u64,
    memory_map_addr: u64,
    memory_map_entries: u64,
    framebuffer: FramebufferInfo,
    rdsp: u64,
    module_count: u64,
    modules: u64,
    epoch: u64,
    flags: StivaleFlags,
}

impl StivaleStructure {
    pub fn cmdline(&self) -> *const u8 {
        unsafe { &*self.inner }.cmdline as *const u8
    }

    pub fn framebuffer(&self) -> &FramebufferInfo {
        &unsafe { &*self.inner }.framebuffer
    }
    
    pub fn rdsp(&self) -> usize {
        unsafe { &*self.inner }.rdsp as usize
    }

    pub fn epoch(&self) -> u64 {
        unsafe { &*self.inner }.epoch
    }

    pub fn flags(&self) -> StivaleFlags {
        unsafe { &*self.inner }.flags
    }
}
