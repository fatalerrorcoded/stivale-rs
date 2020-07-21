use core::marker::PhantomData;

/// The type of the memory map entry
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum MemoryMapEntryType {
    /// Usable memory
    Usable = 1,
    /// Memory reserved by the system
    Reserved = 2,
    /// ACPI memory that can be reclaimed
    AcpiReclaimable = 3,
    /// ACPI memory that cannot be reclaimed
    AcpiNvs = 4,
    /// Memory marked as defective (bad RAM)
    BadMemory = 5,
    /// Memory containing the kernel and any modules
    Kernel = 10,
}

/// A memory region
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryMapEntry {
    base: u64,
    length: u64,
    entry_type: MemoryMapEntryType,
    _unused: u32,
}

impl MemoryMapEntry {
    /// Get the address where the memory region starts
    pub fn start_address(&self) -> u64 {
        self.base
    }

    /// Get the address where the memory region ends
    /// 
    /// Identical to `entry.start_address() + entry.size()`
    pub fn end_address(&self) -> u64 {
        self.base + self.length
    }

    /// Get the size of the memory region
    pub fn size(&self) -> u64 {
        self.length
    }

    /// Get the type of the memory region
    pub fn entry_type(&self) -> MemoryMapEntryType {
        self.entry_type
    }
}

/// An iterator over all memory regions
#[derive(Clone, Debug)]
pub struct MemoryMapIter<'a> {
    addr: *const MemoryMapEntry,
    curr: u64,
    length: u64,
    _phantom: PhantomData<&'a MemoryMapEntry>,
}

impl MemoryMapIter<'_> {
    pub(crate) unsafe fn build(addr: u64, length: u64) -> Self {
        MemoryMapIter {
            addr: addr as *const MemoryMapEntry,
            curr: 0,
            length,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a MemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.length {
            let entry = unsafe { &*self.addr.offset(self.curr as isize) };
            self.curr += 1;
            Some(entry)
        } else {
            None
        }
    }
}
