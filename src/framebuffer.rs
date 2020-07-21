#[repr(packed)]
pub struct FramebufferInfo {
    address: u64,
    pitch: u16,
    width: u16,
    height: u16,
    bpp: u16,
}

impl FramebufferInfo {
    pub fn start_address(&self) -> usize {
        self.address as usize
    }

    pub fn end_address(&self) -> usize {
        self.address as usize + self.size()
    }

    pub fn size(&self) -> usize {
        self.pitch as usize * self.height as usize * (self.bpp as usize / 8)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn pitch(&self) -> u16 {
        self.pitch
    }

    pub fn bpp(&self) -> u16 {
        self.bpp
    }
}
