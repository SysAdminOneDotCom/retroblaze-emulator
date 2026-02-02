use eframe::egui;
use egui::{Vec2, Color32, RichText, Rect};
use std::path::PathBuf;
use crate::library::{GameLibrary, Game};

pub struct LauncherApp {
    library: GameLibrary,
    library_path: PathBuf,
    selected_tab: String,
    selected_game: Option<Game>,
}

impl LauncherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let library_path = PathBuf::from("game_library.json");
        let mut library = GameLibrary::load_from_file(&library_path)
            .unwrap_or_else(|_| GameLibrary::new());
        
        // Scan ROMs directory on startup
        let roms_dir = PathBuf::from("roms");
        let _ = library.scan_roms_directory(&roms_dir);
        let _ = library.save_to_file(&library_path);

        Self {
            library,
            library_path,
            selected_tab: "NES".to_string(),
            selected_game: None,
        }
    }

    fn render_game_grid(&mut self, ui: &mut egui::Ui, system: &str) {
        let games: Vec<Game> = self.library.get_games_by_system(system).into_iter().cloned().collect();
        
        if games.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(RichText::new("No games found").size(18.0).color(Color32::GRAY));
                ui.add_space(10.0);
                ui.label(format!("Add .{} ROM files to the 'roms' folder", 
                    match system {
                        "NES" => "nes",
                        "SNES" => "sfc or .smc",
                        "Genesis" => "gen or .md",
                        _ => "rom",
                    }
                ));
                ui.add_space(10.0);
                if ui.button("Refresh Library").clicked() {
                    let roms_dir = PathBuf::from("roms");
                    let _ = self.library.scan_roms_directory(&roms_dir);
                    let _ = self.library.save_to_file(&self.library_path);
                }
            });
            return;
        }

        // Grid layout for game boxes
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(10.0);
            
            let available_width = ui.available_width();
            let box_width = 180.0;
            let box_height = 250.0;
            let spacing = 20.0;
            let _columns = ((available_width + spacing) / (box_width + spacing)).floor().max(1.0) as usize;

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);
                
                for game in &games {
                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(box_width, box_height),
                        egui::Sense::click()
                    );

                    // Draw game box
                    if ui.is_rect_visible(rect) {
                        // Background
                        ui.painter().rect_filled(
                            rect,
                            5.0,
                            if response.hovered() {
                                Color32::from_rgb(60, 60, 80)
                            } else {
                                Color32::from_rgb(40, 40, 50)
                            }
                        );

                        // Box art placeholder
                        let art_rect = Rect::from_min_size(
                            rect.min + Vec2::new(10.0, 10.0),
                            Vec2::new(box_width - 20.0, 160.0)
                        );
                        self.draw_placeholder(ui, art_rect, &game.title);

                        // Game title
                        let text_rect = Rect::from_min_size(
                            rect.min + Vec2::new(10.0, 180.0),
                            Vec2::new(box_width - 20.0, 60.0)
                        );
                        
                        ui.painter().text(
                            text_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &game.title,
                            egui::FontId::proportional(14.0),
                            Color32::WHITE,
                        );

                        // Border on hover
                        if response.hovered() {
                            ui.painter().rect_stroke(
                                rect,
                                5.0,
                                egui::Stroke::new(2.0, Color32::from_rgb(100, 150, 255))
                            );
                        }

                        // Launch on click
                        if response.clicked() {
                            self.selected_game = Some(game.clone());
                            log::info!("Selected game: {} ({})", game.title, game.system);
                        }
                    }
                }
            });

            ui.add_space(20.0);
        });
    }

    fn draw_placeholder(&self, ui: &mut egui::Ui, rect: Rect, title: &str) {
        ui.painter().rect_filled(rect, 3.0, Color32::from_rgb(30, 30, 40));
        
        // Draw system icon or placeholder text
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(12.0),
            Color32::GRAY,
        );
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if a game was selected and should be launched
        if let Some(game) = self.selected_game.take() {
            self.launch_game(&game);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("🎮 RetroBlazeEmulator").size(28.0));
            });
            ui.add_space(5.0);
            ui.separator();
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Total games: {}", self.library.games.len()));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Refresh Library").clicked() {
                        let roms_dir = PathBuf::from("roms");
                        let _ = self.library.scan_roms_directory(&roms_dir);
                        let _ = self.library.save_to_file(&self.library_path);
                    }
                    if ui.button("Open ROMs Folder").clicked() {
                        let _ = open::that("roms");
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            
            // Tab selection
            let selected_tab = self.selected_tab.clone();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, "NES".to_string(), 
                    RichText::new("🎮 NES").size(16.0));
                ui.selectable_value(&mut self.selected_tab, "SNES".to_string(), 
                    RichText::new("🎮 SNES").size(16.0));
                ui.selectable_value(&mut self.selected_tab, "Genesis".to_string(), 
                    RichText::new("🎮 Genesis").size(16.0));
            });
            
            ui.separator();
            ui.add_space(10.0);

            // Render game grid for selected system
            self.render_game_grid(ui, &selected_tab);
        });
    }
}

impl LauncherApp {
    fn launch_game(&self, game: &Game) {
        use std::process::Command;
        
        let system_arg = game.system.to_lowercase();
        let rom_path = game.rom_path.to_string_lossy().to_string();
        
        log::info!("Launching: {} - {}", game.title, rom_path);
        
        // Launch the emulator in a separate process
        let _ = Command::new(std::env::current_exe().unwrap())
            .arg("--system")
            .arg(system_arg)
            .arg("--rom")
            .arg(rom_path)
            .spawn();
    }
}
