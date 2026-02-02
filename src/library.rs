use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub system: String,  // "NES", "SNES", "Genesis"
    pub rom_path: PathBuf,
    pub box_art_path: Option<PathBuf>,
    pub description: Option<String>,
    pub year: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameLibrary {
    pub games: Vec<Game>,
}

impl GameLibrary {
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let library = serde_json::from_str(&data)?;
            Ok(library)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn get_games_by_system(&self, system: &str) -> Vec<&Game> {
        self.games.iter().filter(|g| g.system == system).collect()
    }

    pub fn scan_roms_directory(&mut self, roms_dir: &Path) -> Result<()> {
        if !roms_dir.exists() {
            fs::create_dir_all(roms_dir)?;
            return Ok(());
        }

        for entry in fs::read_dir(roms_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    let system = match ext.as_str() {
                        "nes" => "NES",
                        "sfc" | "smc" => "SNES",
                        "gen" | "md" => "Genesis",
                        _ => continue,
                    };

                    // Check if game already exists
                    let already_exists = self.games.iter().any(|g| g.rom_path == path);
                    if already_exists {
                        continue;
                    }

                    let title = path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // Look for matching box art
                    let box_art_path = roms_dir.join("boxart").join(format!("{}.png", title));
                    let box_art = if box_art_path.exists() {
                        Some(box_art_path)
                    } else {
                        None
                    };

                    self.add_game(Game {
                        title,
                        system: system.to_string(),
                        rom_path: path,
                        box_art_path: box_art,
                        description: None,
                        year: None,
                    });
                }
            }
        }

        Ok(())
    }
}
