use anyhow::Result;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, FullscreenType};

pub struct Renderer {
    canvas: Canvas<Window>,
    current_width: u32,
    current_height: u32,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Self> {
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()?;
        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        
        Ok(Self {
            canvas,
            current_width: 256,
            current_height: 240,
        })
    }
    
    pub fn render(&mut self, framebuffer: &[u8]) -> Result<()> {
        // Determine framebuffer dimensions
        let (width, height) = match framebuffer.len() {
            246_784 => (256, 240),  // NES: 256x240x4
            229_376 => (256, 224),  // SNES: 256x224x4
            286_720 => (320, 224),  // Genesis: 320x224x4
            _ => (256, 240),        // Default to NES
        };
        
        // Create texture for this frame
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, width, height)?;
        
        // Update texture with framebuffer data
        texture.update(None, framebuffer, (width * 4) as usize)
            .map_err(|e| anyhow::anyhow!("Texture update failed: {}", e))?;
        
        // Clear canvas
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        
        // Render texture with aspect ratio preservation
        let (window_width, window_height) = self.canvas.output_size()
            .map_err(|e| anyhow::anyhow!("Failed to get window size: {}", e))?;
        let aspect_ratio = width as f32 / height as f32;
        let window_aspect = window_width as f32 / window_height as f32;
        
        let (dst_width, dst_height) = if window_aspect > aspect_ratio {
            // Window is wider - fit to height
            let h = window_height;
            let w = (h as f32 * aspect_ratio) as u32;
            (w, h)
        } else {
            // Window is taller - fit to width
            let w = window_width;
            let h = (w as f32 / aspect_ratio) as u32;
            (w, h)
        };
        
        let x = (window_width - dst_width) / 2;
        let y = (window_height - dst_height) / 2;
        
        let dst_rect = Rect::new(x as i32, y as i32, dst_width, dst_height);
        self.canvas.copy(&texture, None, Some(dst_rect))
            .map_err(|e| anyhow::anyhow!("Failed to copy texture: {}", e))?;
        
        self.canvas.present();
        Ok(())
    }
    
    pub fn toggle_fullscreen(&mut self) -> Result<()> {
        let window = self.canvas.window_mut();
        let current_state = window.fullscreen_state();
        
        let new_state = match current_state {
            FullscreenType::Off => FullscreenType::Desktop,
            _ => FullscreenType::Off,
        };
        
        window.set_fullscreen(new_state)
            .map_err(|e| anyhow::anyhow!("Failed to toggle fullscreen: {}", e))?;
        
        Ok(())
    }
}
