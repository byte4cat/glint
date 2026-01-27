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

    // pos
    pub margin_top: Option<i32>,
    pub margin_bottom: Option<i32>,
    pub margin_left: Option<i32>,
    pub margin_right: Option<i32>,

    // css
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

fn default_bg() -> String {
    "#1e1e2e".into()
}
fn default_text() -> String {
    "#cdd6f4".into()
}
fn default_font_size() -> u32 {
    14
}
fn default_title_size() -> u32 {
    24
}
fn default_border_width() -> u32 {
    3
}
fn default_border_radius() -> u32 {
    16
}
fn default_border_color() -> String {
    "#89b4fa".into()
}
fn default_font_family() -> Vec<String> {
    vec!["JetBrains Mono".into(), "sans-serif".into()]
}
fn default_monitor_name() -> Option<String> {
    None
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
        let mut config: Config = toml::from_str(&content)?;

        // default to right bottom
        if config.margin_top.is_none()
            && config.margin_bottom.is_none()
            && config.margin_left.is_none()
            && config.margin_right.is_none()
        {
            config.margin_bottom = Some(40);
            config.margin_right = Some(40);
        }

        Ok(config)
    }
}
