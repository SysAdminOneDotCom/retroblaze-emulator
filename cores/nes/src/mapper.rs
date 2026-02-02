/// NES Cartridge Mappers
/// Different games use different memory mappers to expand ROM/RAM

pub trait Mapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

/// Mapper 0 - NROM (No mapper, direct mapping)
pub struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Mapper0 {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        Self { prg_rom, chr_rom }
    }
}

impl Mapper for Mapper0 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => {
                let addr = (addr - 0x8000) as usize;
                if addr < self.prg_rom.len() {
                    self.prg_rom[addr]
                } else {
                    self.prg_rom[addr % self.prg_rom.len()]
                }
            }
            _ => 0,
        }
    }
    
    fn write(&mut self, _addr: u16, _value: u8) {
        // NROM is read-only
    }
}
