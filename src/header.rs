bitflags! {
    pub struct StivaleHeaderFlags: u64 {
        const FRAMEBUFFER_MODE = 0x1;
        const FIVE_LEVEL_PAGING = 0x2;
        const KASLR = 0x4;
    }
}

#[link_section = ".stivalehdr"]
#[allow(dead_code)]
pub struct StivaleHeader {
    stack: u64,
    flags: StivaleHeaderFlags,
    framebuffer_width: u16,
    framebuffer_height: u16,
    framebuffer_bpp: u16,
    entry_point: u64,
}

impl StivaleHeader {
    pub const fn new(stack: *const u8, flags: StivaleHeaderFlags) -> Self {
        StivaleHeader {
            stack: unsafe { stack as u64 },
            flags,
            framebuffer_width: 0,
            framebuffer_height: 0,
            framebuffer_bpp: 0,
            entry_point: 0
        }
    }

    pub const fn with_entry_point(mut self, entry_point: fn(stivale_struct: usize)) -> Self {
        self.entry_point = unsafe { entry_point as u64 };
        self
    }

    pub const fn with_framebuffer(mut self, width: u16, height: u16, bpp: u16) -> Self {
        self.framebuffer_width = width;
        self.framebuffer_height = height;
        self.framebuffer_bpp = bpp;
        self
    }
}
