use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Default)]
pub struct Config {
    pub title: String,
    pub tasks: Vec<String>,
    pub anchor_bottom: bool,
    pub anchor_right: bool,
    pub margin_bottom: i32,
    pub margin_right: i32,
    pub background_color: String,
    pub text_color: String,
    pub font_size: u32,
}

impl Config {
    fn get_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("glint/config.toml");
        path
    }

    pub fn load() -> Self {
        fs::read_to_string(Self::get_path())
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_else(Self::default_config)
    }

    fn default_config() -> Self {
        Self {
            title: "✨ GLINT".to_string(),
            tasks: vec!["Create ~/.config/glint/config.toml".to_string()],
            anchor_bottom: true,
            anchor_right: true,
            margin_bottom: 40,
            margin_right: 40,
            background_color: "#1e1e2e".to_string(),
            text_color: "#cdd6f4".to_string(),
            font_size: 14,
        }
    }

    pub fn format_markdown(&self) -> String {
        format!("<b>{}</b>\n\n{}", self.title, self.tasks.join("\n"))
    }
}
