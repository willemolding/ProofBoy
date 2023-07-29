pub trait MemoryReader {
    /// Reads one byte from the given address in the memory.
    fn get8(&self, addr: u16) -> u8;
    /// Reads two bytes from the given addresss in the memory.
    fn get16(&self, addr: u16) -> u16;
}

impl<D> MemoryReader for rgy::System<D>
where
    D: rgy::debug::Debugger + 'static,
{
    fn get8(&self, addr: u16) -> u8 {
        self.mmu_get8(addr)
    }

    fn get16(&self, addr: u16) -> u16 {
        self.mmu_get16(addr)
    }
}
