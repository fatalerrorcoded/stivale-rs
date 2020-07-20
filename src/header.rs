bitflags! {
    pub struct StivaleHeaderFlags: u64 {
        const FRAMEBUFFER_MODE = 0x1;
        const FIVE_LEVEL_PAGING = 0x2;
        const KASLR = 0x4;
    }
}

#[link_section = ".stivalehdr"]
pub struct StivaleHeader {
    pub stack: u64,
    pub flags: StivaleHeaderFlags,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_bpp: u16,
    pub entry_point: u64,
}
