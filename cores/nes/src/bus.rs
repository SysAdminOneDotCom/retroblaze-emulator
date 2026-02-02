/// NES Memory Bus
/// Handles memory mapping and cartridge access

use anyhow::Result;

pub struct Bus {
    // Internal RAM (2KB, mirrored to 0x2000)
    ram: [u8; 0x800],
    
    // Cartridge ROM/RAM
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x800],
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
        }
    }
    
    pub fn load_cartridge(&mut self, rom_data: &[u8]) -> Result<()> {
        // Parse iNES format header
        if rom_data.len() < 16 {
            anyhow::bail!("ROM too small");
        }
        
        // Check for "NES\x1A" magic number
        if &rom_data[0..4] != b"NES\x1A" {
            anyhow::bail!("Invalid NES ROM format");
        }
        
        let prg_rom_size = rom_data[4] as usize * 16384; // 16KB units
        let chr_rom_size = rom_data[5] as usize * 8192;  // 8KB units
        
        let prg_start = 16; // After header
        let chr_start = prg_start + prg_rom_size;
        
        self.prg_rom = rom_data[prg_start..prg_start + prg_rom_size].to_vec();
        
        if chr_rom_size > 0 {
            self.chr_rom = rom_data[chr_start..chr_start + chr_rom_size].to_vec();
        }
        
        log::info!("Loaded NES ROM: PRG={} KB, CHR={} KB", 
                   prg_rom_size / 1024, chr_rom_size / 1024);
        
        Ok(())
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // RAM (mirrored)
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize],
            
            // PPU registers (mirrored)
            0x2000..=0x3FFF => {
                // TODO: Read from PPU
                0
            }
            
            // APU and I/O registers
            0x4000..=0x4017 => {
                // TODO: Read from APU/controller
                0
            }
            
            // Cartridge space
            0x8000..=0xFFFF => {
                if self.prg_rom.is_empty() {
                    0
                } else {
                    let addr = (addr - 0x8000) as usize;
                    if addr < self.prg_rom.len() {
                        self.prg_rom[addr]
                    } else {
                        // Mirror if ROM is 16KB
                        self.prg_rom[addr % self.prg_rom.len()]
                    }
                }
            }
            
            _ => 0,
        }
    }
    
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM (mirrored)
            0x0000..=0x1FFF => {
                self.ram[(addr & 0x07FF) as usize] = value;
            }
            
            // PPU registers (mirrored)
            0x2000..=0x3FFF => {
                // TODO: Write to PPU
            }
            
            // APU and I/O registers
            0x4000..=0x4017 => {
                // TODO: Write to APU/controller
            }
            
            // Cartridge space (usually ROM, but some mappers allow writes)
            0x8000..=0xFFFF => {
                // TODO: Handle mapper writes
            }
            
            _ => {}
        }
    }
}
