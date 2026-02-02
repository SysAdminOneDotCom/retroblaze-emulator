use anyhow::Result;
use std::path::Path;
use crate::input_state::InputState;

// Import emulator cores
use nes_core::NES;
use snes_core::SNES;
use genesis_core::Genesis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemType {
    NES,
    SNES,
    Genesis,
}

pub trait EmulatorCore {
    fn reset(&mut self);
    fn run_frame(&mut self, input: &InputState) -> Result<()>;
    fn get_framebuffer(&self) -> &[u8];
    fn get_audio_samples(&mut self) -> &[i16];
    fn save_state(&self) -> Result<Vec<u8>>;
    fn load_state(&mut self, data: &[u8]) -> Result<()>;
}

pub struct Emulator {
    system_type: SystemType,
    core: Box<dyn EmulatorCore>,
}

impl Emulator {
    pub fn new(system_type: SystemType) -> Result<Self> {
        let core: Box<dyn EmulatorCore> = match system_type {
            SystemType::NES => {
                Box::new(NESCore::new())
            }
            SystemType::SNES => {
                Box::new(SNESCore::new())
            }
            SystemType::Genesis => {
                Box::new(GenesisCore::new())
            }
        };
        
        Ok(Self {
            system_type,
            core,
        })
    }
    
    pub fn load_rom(&mut self, _path: &Path) -> Result<()> {
        // ROM loading will be implemented when cores are complete
        Ok(())
    }
    
    pub fn run_frame(&mut self, input: &InputState) -> Result<()> {
        self.core.run_frame(input)
    }
    
    pub fn get_framebuffer(&self) -> &[u8] {
        self.core.get_framebuffer()
    }
    
    pub fn save_state(&self, path: &Path) -> Result<()> {
        let state_data = self.core.save_state()?;
        std::fs::write(path, state_data)?;
        Ok(())
    }
    
    pub fn load_state(&mut self, path: &Path) -> Result<()> {
        let state_data = std::fs::read(path)?;
        self.core.load_state(&state_data)?;
        Ok(())
    }
}

// Placeholder cores (to be implemented in separate modules)
struct NESCore {
    nes: NES,
}

impl NESCore {
    fn new() -> Self {
        Self {
            nes: NES::new(),
        }
    }
    
    fn load_rom(&mut self, data: &[u8]) -> Result<()> {
        self.nes.load_rom(data)
    }
}

impl EmulatorCore for NESCore {
    fn reset(&mut self) {
        self.nes.reset();
    }
    
    fn run_frame(&mut self, _input: &InputState) -> Result<()> {
        // Run one frame worth of emulation
        self.nes.run_frame();
        Ok(())
    }
    
    fn get_framebuffer(&self) -> &[u8] {
        self.nes.ppu.get_framebuffer()
    }
    
    fn get_audio_samples(&mut self) -> &[i16] {
        // TODO: Get audio from APU
        &[]
    }
    
    fn save_state(&self) -> Result<Vec<u8>> {
        // TODO: Serialize emulator state
        Ok(Vec::new())
    }
    
    fn load_state(&mut self, _data: &[u8]) -> Result<()> {
        // TODO: Deserialize emulator state
        Ok(())
    }
}

struct SNESCore {
    snes: SNES,
}

impl SNESCore {
    fn new() -> Self {
        Self {
            snes: SNES::new(),
        }
    }
    
    fn load_rom(&mut self, data: &[u8]) -> Result<()> {
        self.snes.load_rom(data)
    }
}

impl EmulatorCore for SNESCore {
    fn reset(&mut self) {
        self.snes.reset();
    }
    
    fn run_frame(&mut self, _input: &InputState) -> Result<()> {
        self.snes.run_frame();
        Ok(())
    }
    
    fn get_framebuffer(&self) -> &[u8] {
        self.snes.get_framebuffer()
    }
    
    fn get_audio_samples(&mut self) -> &[i16] {
        &[]
    }
    
    fn save_state(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    
    fn load_state(&mut self, _data: &[u8]) -> Result<()> {
        Ok(())
    }
}

struct GenesisCore {
    genesis: Genesis,
}

impl GenesisCore {
    fn new() -> Self {
        Self {
            genesis: Genesis::new(),
        }
    }
    
    fn load_rom(&mut self, data: &[u8]) -> Result<()> {
        self.genesis.load_rom(data)
    }
}

impl EmulatorCore for GenesisCore {
    fn reset(&mut self) {
        self.genesis.reset();
    }
    
    fn run_frame(&mut self, _input: &InputState) -> Result<()> {
        self.genesis.run_frame();
        Ok(())
    }
    
    fn get_framebuffer(&self) -> &[u8] {
        self.genesis.get_framebuffer()
    }
    
    fn get_audio_samples(&mut self) -> &[i16] {
        &[]
    }
    
    fn save_state(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    
    fn load_state(&mut self, _data: &[u8]) -> Result<()> {
        Ok(())
    }
}


