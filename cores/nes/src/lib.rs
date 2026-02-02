/// NES (Nintendo Entertainment System) Emulator Core
/// 
/// Architecture:
/// - CPU: MOS Technology 6502 @ 1.79 MHz
/// - PPU: Picture Processing Unit (2C02)
/// - APU: Audio Processing Unit (5 channels)
/// - Memory: 2KB RAM + cartridge ROM/RAM

pub mod cpu;
pub mod ppu;
pub mod apu;
pub mod mapper;
pub mod bus;

use anyhow::Result;

pub struct NES {
    pub cpu: cpu::CPU6502,
    pub ppu: ppu::PPU,
    pub apu: apu::APU,
    pub bus: bus::Bus,
    cycles: u64,
}

impl NES {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CPU6502::new(),
            ppu: ppu::PPU::new(),
            apu: apu::APU::new(),
            bus: bus::Bus::new(),
            cycles: 0,
        }
    }
    
    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<()> {
        self.bus.load_cartridge(rom_data)?;
        self.reset();
        Ok(())
    }
    
    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.bus);
        self.ppu.reset();
        self.apu.reset();
        self.cycles = 0;
    }
    
    pub fn step(&mut self) -> u8 {
        // Execute one CPU instruction
        let cpu_cycles = self.cpu.step(&mut self.bus);
        
        // PPU runs 3 times faster than CPU
        for _ in 0..(cpu_cycles * 3) {
            self.ppu.step(&mut self.bus);
        }
        
        // APU runs at CPU speed
        for _ in 0..cpu_cycles {
            self.apu.step();
        }
        
        self.cycles += cpu_cycles as u64;
        cpu_cycles
    }
    
    pub fn run_frame(&mut self) {
        // NES runs at ~60 Hz, 1 frame = 29780.5 CPU cycles
        const CYCLES_PER_FRAME: u32 = 29781;
        let target = self.cycles + CYCLES_PER_FRAME as u64;
        
        while self.cycles < target {
            self.step();
        }
    }
}
