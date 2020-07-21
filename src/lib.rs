#![no_std]
#![warn(clippy::all)]

//! A crate for parsing qloader2 and tomatboot's stivale structures

#[macro_use]
extern crate bitflags;

pub mod header;
pub use header::{StivaleHeader, StivaleHeaderFlags};

pub mod framebuffer;
pub use framebuffer::FramebufferInfo;

pub mod memory;
pub mod module;
use memory::MemoryMapIter;
use module::ModuleIter;

/// Checks if the value matches the ASCII "stivale!" signature (0x73746976616c6521)
///
/// The signature "stivale!" (0x73746976616c6521) is passed in the EAX register
///
/// Example
///
/// ```ignore
/// let stivale_signature: u64;
/// unsafe { asm!("mov $2, %eax" : "=r"(stivale_signature)) };
/// println("{}", stivale::check_signature(stivale_signature))
/// ```
pub fn check_signature(signature: u64) -> bool {
    signature == 0x73746976616c6521
}

/// Load the stivale structure from an address
///
/// The structure pointer is passed in the EDI register
///
/// Examples
///
/// ```ignore
/// let stivale_struct_ptr: u64;
/// unsafe { asm!("mov $2, %rax" : "=r"(stivale_struct_ptr)) };
/// let stivale_struct = unsafe { stivale::load(stivale_struct_ptr as usize) };
/// ```
///
/// ```ignore
/// fn kernel_main(stivale_struct_ptr: usize) {
///     let stivale_struct = unsafe { stivale::load(stivale_struct_ptr) };
/// }
/// ```
#[allow(clippy::missing_safety_doc)]
pub unsafe fn load(address: usize) -> StivaleStructure {
    let inner = &*(address as *const StivaleStructureInner);
    StivaleStructure { inner }
}

bitflags! {
    pub struct StivaleFlags: u64 {
        const BIOS_BOOT = 0x1;
    }
}

/// The stivale structure
pub struct StivaleStructure {
    inner: *const StivaleStructureInner,
}

#[repr(packed)]
struct StivaleStructureInner {
    cmdline: u64,
    memory_map_addr: u64,
    memory_map_entries: u64,
    framebuffer: FramebufferInfo,
    rsdp: u64,
    module_count: u64,
    modules: u64,
    epoch: u64,
    flags: StivaleFlags,
}

impl StivaleStructure {
    fn inner(&self) -> &StivaleStructureInner {
        unsafe { &*self.inner }
    }

    /// Get the command line passed in the structure by the bootloader, if any
    pub fn cmdline(&self) -> Option<&str> {
        use core::{slice, str};
        let cmdline = self.inner().cmdline as *const u8;
        if unsafe { *cmdline } == 0 {
            None
        } else {
            let mut strlen = 0;
            while unsafe { *cmdline.offset(strlen) } != 0 {
                strlen += 1;
            }

            unsafe {
                Some(str::from_utf8_unchecked(slice::from_raw_parts(
                    cmdline,
                    strlen as usize,
                )))
            }
        }
    }

    /// Get the video framebuffer info
    pub fn framebuffer(&self) -> &FramebufferInfo {
        &self.inner().framebuffer
    }

    /// Get the ACPI RSDP structure pointer
    pub fn rsdp(&self) -> usize {
        self.inner().rsdp as usize
    }

    /// Get the current UNIX epoch during boot
    pub fn epoch(&self) -> u64 {
        self.inner().epoch
    }

    /// Get the flags passed by the bootloader
    pub fn flags(&self) -> StivaleFlags {
        self.inner().flags
    }

    /// Get an iterator over the memory map
    pub fn memory_map_iter(&self) -> MemoryMapIter {
        unsafe {
            MemoryMapIter::build(
                self.inner().memory_map_addr,
                self.inner().memory_map_entries,
            )
        }
    }

    /// Get an iterator over all the modules loaded by the bootloader
    pub fn module_iter(&self) -> ModuleIter {
        unsafe { ModuleIter::build(self.inner().modules, self.inner().module_count) }
    }
}
