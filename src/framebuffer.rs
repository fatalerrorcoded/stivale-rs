/// The framebuffer info passed by the bootloader
/// and based on the configuration in the stivale header
/// 
/// May be in an undefined state if a textual framebuffer is configured
#[repr(packed)]
pub struct FramebufferInfo {
    address: u64,
    pitch: u16,
    width: u16,
    height: u16,
    bpp: u16,
}

impl FramebufferInfo {
    /// Get the start address of the framebuffer
    pub fn start_address(&self) -> usize {
        self.address as usize
    }

    /// Get the end address of the framebuffer
    /// 
    /// Identical to `framebuffer_info.start_address() + framebuffer_info.size()`
    pub fn end_address(&self) -> usize {
        self.address as usize + self.size()
    }

    /// Get the size of the framebuffer
    pub fn size(&self) -> usize {
        self.pitch as usize * self.height as usize * (self.bpp as usize / 8)
    }

    /// Get the width of the framebuffer in pixels
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the height of the framebuffer in pixels
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get the bytes per line of the framebuffer
    pub fn pitch(&self) -> u16 {
        self.pitch
    }

    /// Get the bits per pixel of the framebuffer
    pub fn bpp(&self) -> u16 {
        self.bpp
    }
}
