use core::fmt;
use core::marker::PhantomData;

/// A module info struct
#[repr(packed)]
pub struct Module {
    start: u64,
    end: u64,
    string: [u8; 128],
    next: u64,
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We copy as borrowing from packed structs is an error
        let start = self.start;
        let end = self.end;
        f.debug_struct("Module")
            .field("start", &start)
            .field("end", &end)
            .field("string", &self.string())
            .finish()
    }
}

impl Module {
    /// Get the address where the module starts
    pub fn start_address(&self) -> u64 {
        self.start
    }

    /// Get the address where the module ends
    pub fn end_address(&self) -> u64 {
        self.end
    }

    /// Get the size of the module
    /// 
    /// Identical to `module.start_address() + module.end_address()`
    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    /// Get the string passed to the module by the bootloader, if any
    pub fn string(&self) -> Option<&str> {
        use core::{slice, str};
        if self.string[0] == 0 {
            None
        } else {
            let mut strlen = 0;
            while strlen < 128 && self.string[strlen] != 0 {
                strlen += 1;
            }

            unsafe {
                Some(str::from_utf8_unchecked(slice::from_raw_parts(
                    (&self.string[0]) as *const u8,
                    strlen,
                )))
            }
        }
    }

    fn next(&self) -> *const Self {
        self.next as *const Self
    }
}

/// An iterator over all the loaded modules
#[derive(Clone, Debug)]
pub struct ModuleIter<'a> {
    next: *const Module,
    curr: u64,
    length: u64,
    _phantom: PhantomData<&'a Module>,
}

impl ModuleIter<'_> {
    pub(crate) unsafe fn build(addr: u64, length: u64) -> Self {
        ModuleIter {
            next: addr as *const Module,
            curr: 0,
            length,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'a> Iterator for ModuleIter<'a> {
    type Item = &'a Module;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.length {
            let module = unsafe { &*self.next };
            self.curr += 1;
            self.next = module.next();
            Some(module)
        } else {
            None
        }
    }
}
