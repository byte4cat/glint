use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Config {
    #[serde(default = "default_note_path")]
    pub note_path: String,
    #[serde(default = "default_title_size")]
    pub title_size: u32,
    #[serde(default = "default_monitor_name")]
    pub monitor_name: Option<String>,
    #[serde(default = "default_true")]
    pub anchor_bottom: bool,
    #[serde(default = "default_true")]
    pub anchor_right: bool,
    #[serde(default = "default_margin")]
    pub margin_bottom: i32,
    #[serde(default = "default_margin")]
    pub margin_right: i32,
    #[serde(default = "default_bg")]
    pub background_color: String,
    #[serde(default = "default_text")]
    pub text_color: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_font_family")]
    pub font_family: Vec<String>,
    #[serde(default = "default_border_width")]
    pub border_width: u32,
    #[serde(default = "default_border_color")]
    pub border_color: String,
    #[serde(default = "default_border_radius")]
    pub border_radius: u32,
}

fn default_title_size() -> u32 {
    24
}

fn default_monitor_name() -> Option<String> {
    None
}

fn default_true() -> bool {
    true
}

fn default_margin() -> i32 {
    40
}

fn default_bg() -> String {
    "#1e1e2e".to_string()
}

fn default_text() -> String {
    "#cdd6f4".to_string()
}

fn default_font_size() -> u32 {
    14
}

fn default_font_family() -> Vec<String> {
    vec!["JetBrains Mono".into(), "sans-serif".into()]
}

fn default_border_width() -> u32 {
    3
}

fn default_border_color() -> String {
    "#89b4fa".to_string()
}

fn default_border_radius() -> u32 {
    16
}

fn default_note_path() -> String {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("glint");
    path.push("notes.md");
    path.to_string_lossy().to_string()
}

impl Config {
    pub fn get_note_content(&self) -> String {
        let path = shellexpand::tilde(&self.note_path).to_string();
        println!("note path should be: {}", path);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| format!("# ✨Glint\n\nPlease write your note at `{}`", path))
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("glint")
            .join("config.toml");

        let content = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
