use anyhow::Result;
use log::{info, warn};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod audio;
mod emulator;
mod input;
mod input_state;
mod video;
mod utils;
mod library;
mod launcher;

use emulator::{Emulator, SystemType};
use input::ControllerManager;
use input_state::InputState;
use video::Renderer;

/// Command line arguments
#[derive(Debug)]
struct Args {
    system: Option<SystemType>,
    rom_path: Option<PathBuf>,
    state_path: Option<PathBuf>,
    debug: bool,
    launcher_mode: bool,
}

fn parse_args() -> Result<Args> {
    let args: Vec<String> = std::env::args().collect();
    
    // If no arguments, launch in GUI mode
    if args.len() == 1 {
        return Ok(Args {
            system: None,
            rom_path: None,
            state_path: None,
            debug: false,
            launcher_mode: true,
        });
    }
    
    let mut system = None;
    let mut rom_path = None;
    let mut state_path = None;
    let mut debug = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--system" => {
                i += 1;
                system = Some(match args[i].as_str() {
                    "nes" => SystemType::NES,
                    "snes" => SystemType::SNES,
                    "genesis" | "md" => SystemType::Genesis,
                    _ => anyhow::bail!("Unknown system: {}", args[i]),
                });
            }
            "--rom" => {
                i += 1;
                rom_path = Some(PathBuf::from(&args[i]));
            }
            "--state" => {
                i += 1;
                state_path = Some(PathBuf::from(&args[i]));
            }
            "--debug" => {
                debug = true;
            }
            "--launcher" => {
                return Ok(Args {
                    system: None,
                    rom_path: None,
                    state_path: None,
                    debug,
                    launcher_mode: true,
                });
            }
            _ => {}
        }
        i += 1;
    }
    
    if rom_path.is_none() || system.is_none() {
        anyhow::bail!("Usage: {} --system <nes|snes|genesis> --rom <path>", args[0]);
    }
    
    let rom = rom_path.unwrap();
    if !rom.exists() {
        anyhow::bail!("ROM file not found: {:?}", rom);
    }
    
    Ok(Args {
        system,
        rom_path: Some(rom),
        state_path,
        debug,
        launcher_mode: false,
    })
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("🎮 RetroBlazeEmulator Starting...");
    
    // Parse arguments
    let args = parse_args()?;
    
    // Launch GUI if in launcher mode
    if args.launcher_mode {
        return launch_gui();
    }
    
    // Otherwise launch emulator directly
    let system = args.system.unwrap();
    let rom_path = args.rom_path.unwrap();
    
    info!("System: {:?}, ROM: {:?}", system, rom_path);
    
    run_emulator(system, rom_path, args.state_path)
}

fn launch_gui() -> Result<()> {
    use launcher::LauncherApp;
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("RetroBlazeEmulator - Game Library"),
        ..Default::default()
    };
    
    eframe::run_native(
        "RetroBlazeEmulator",
        native_options,
        Box::new(|cc| Box::new(LauncherApp::new(cc))),
    ).map_err(|e| anyhow::anyhow!("GUI error: {}", e))?;
    
    Ok(())
}

fn run_emulator(system: SystemType, rom_path: PathBuf, state_path: Option<PathBuf>) -> Result<()> {
    
    // Initialize SDL2
    let sdl_context = sdl2::init().map_err(|e| anyhow::anyhow!("SDL2 init failed: {}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow::anyhow!("Video init failed: {}", e))?;
    let _audio_subsystem = sdl_context.audio().map_err(|e| anyhow::anyhow!("Audio init failed: {}", e))?;
    
    // Create window
    let window_title = format!("RetroBlazeEmulator - {}", rom_path.file_name().unwrap().to_string_lossy());
    let window = video_subsystem
        .window(&window_title, 800, 600)
        .position_centered()
        .resizable()
        .build()?;
    
    // Initialize renderer
    let mut renderer = Renderer::new(window)?;
    
    // Initialize controller manager
    let mut controller_manager = ControllerManager::new()?;
    info!("Controller support initialized - Looking for PlayStation DualShock 4...");
    
    // Load emulator
    let mut emulator = Emulator::new(system)?;
    emulator.load_rom(&rom_path)?;
    
    if let Some(ref save_state_path) = state_path {
        info!("Loading save state: {:?}", save_state_path);
        emulator.load_state(save_state_path)?;
    }
    
    info!("✅ Emulator initialized successfully!");
    info!("Controls:");
    info!("  ESC - Quit");
    info!("  F5 - Save State");
    info!("  F9 - Load State");
    info!("  F11 - Toggle Fullscreen");
    info!("  PS4 Controller - Auto-detected if connected");
    
    // Main loop
    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow::anyhow!("Event pump failed: {}", e))?;
    let target_fps = 60.0;
    let frame_duration = Duration::from_secs_f64(1.0 / target_fps);
    
    let mut running = true;
    let mut paused = false;
    let mut frame_count = 0u64;
    let mut last_fps_time = Instant::now();
    let mut _fps = 0.0;
    let debug = false;
    
    while running {
        let frame_start = Instant::now();
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Escape => running = false,
                        Keycode::F5 => {
                            info!("Saving state...");
                            let state_path = PathBuf::from("quicksave.state");
                            if let Err(e) = emulator.save_state(&state_path) {
                                warn!("Failed to save state: {}", e);
                            } else {
                                info!("✅ State saved!");
                            }
                        }
                        Keycode::F9 => {
                            info!("Loading state...");
                            let state_path = PathBuf::from("quicksave.state");
                            if let Err(e) = emulator.load_state(&state_path) {
                                warn!("Failed to load state: {}", e);
                            } else {
                                info!("✅ State loaded!");
                            }
                        }
                        Keycode::F11 => {
                            renderer.toggle_fullscreen()?;
                        }
                        Keycode::P => {
                            paused = !paused;
                            info!("{}", if paused { "⏸ Paused" } else { "▶ Resumed" });
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // Update controller input
        controller_manager.update();
        let input_state = controller_manager.get_state();
        
        // Run emulation frame
        if !paused {
            emulator.run_frame(&input_state)?;
        }
        
        // Render
        let framebuffer = emulator.get_framebuffer();
        renderer.render(framebuffer)?;
        
        // FPS counter
        frame_count += 1;
        if last_fps_time.elapsed() >= Duration::from_secs(1) {
            _fps = frame_count as f64 / last_fps_time.elapsed().as_secs_f64();
            frame_count = 0;
            last_fps_time = Instant::now();
            if debug {
                info!("FPS: {:.2}", _fps);
            }
        }
        
        // Frame limiting
        let frame_time = frame_start.elapsed();
        if frame_time < frame_duration {
            std::thread::sleep(frame_duration - frame_time);
        }
    }
    
    info!("👋 Shutting down...");
    Ok(())
}
