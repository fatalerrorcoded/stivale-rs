#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MemoryMapEntryType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    Kernel = 10,
}

#[repr(packed)]
pub struct MemoryMapEntry {
    base: u64,
    length: u64,
    entry_type: MemoryMapEntryType,
    _unused: u32,
}

impl MemoryMapEntry {
    pub fn start_address(&self) -> u64 {
        self.base
    }

    pub fn end_address(&self) -> u64 {
        self.base + self.length
    }

    pub fn size(&self) -> u64 {
        self.length
    }

    pub fn entry_type(&self) -> MemoryMapEntryType {
        self.entry_type
    }
}
