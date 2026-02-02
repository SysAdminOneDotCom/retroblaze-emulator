/// NES Picture Processing Unit (PPU)
/// Handles graphics rendering

pub struct PPU {
    // VRAM
    vram: [u8; 0x4000],
    oam: [u8; 256],  // Object Attribute Memory (sprites)
    
    // Palette
    palette: [u8; 32],
    
    // Registers
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    
    // Internal
    scanline: u16,
    cycle: u16,
    
    // Frame buffer (256x240 RGBA)
    pub framebuffer: Vec<u8>,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x4000],
            oam: [0; 256],
            palette: [0; 32],
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            scanline: 0,
            cycle: 0,
            framebuffer: vec![0; 256 * 240 * 4],
        }
    }
    
    pub fn reset(&mut self) {
        self.ctrl = 0;
        self.mask = 0;
        self.status = 0;
        self.oam_addr = 0;
        self.scanline = 0;
        self.cycle = 0;
    }
    
    pub fn step(&mut self, _bus: &mut crate::bus::Bus) {
        self.cycle += 1;
        
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            
            if self.scanline > 261 {
                self.scanline = 0;
                // Frame complete - render test pattern
                self.render_test_pattern();
            }
        }
    }
    
    fn render_test_pattern(&mut self) {
        // Create a colorful test pattern to verify rendering works
        for y in 0..240 {
            for x in 0..256 {
                let pixel_index = (y * 256 + x) * 4;
                
                // Create gradient pattern
                let r = ((x as f32 / 256.0) * 255.0) as u8;
                let g = ((y as f32 / 240.0) * 255.0) as u8;
                let b = (((x + y) as f32 / 496.0) * 255.0) as u8;
                
                // Add some checkerboard pattern
                let checker = if (x / 8 + y / 8) % 2 == 0 { 50 } else { 0 };
                
                self.framebuffer[pixel_index] = r.saturating_add(checker);
                self.framebuffer[pixel_index + 1] = g.saturating_add(checker);
                self.framebuffer[pixel_index + 2] = b.saturating_add(checker);
                self.framebuffer[pixel_index + 3] = 255;
            }
        }
    }
    
    pub fn get_framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}
