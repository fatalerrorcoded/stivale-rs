use core::marker::PhantomData;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum MemoryMapEntryType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    Kernel = 10,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
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
