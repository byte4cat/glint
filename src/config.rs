use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Config {
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_tasks")]
    pub tasks: Vec<String>,
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
    #[serde(default = "default_border_width")]
    pub border_width: u32,
    #[serde(default = "default_border_color")]
    pub border_color: String,
    #[serde(default = "default_border_radius")]
    pub border_radius: u32,
}

fn default_title() -> String {
    "✨ GLINT".to_string()
}

fn default_tasks() -> Vec<String> {
    vec![
        "⚠️ No valid config found, using defaults.".to_string(),
        "Please Check example config at example/config.toml in our github repository".to_string(),
        "🐧 ~/.config/glint/config.toml".to_string(),
    ]
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

fn default_border_width() -> u32 {
    3
}

fn default_border_color() -> String {
    "#89b4fa".to_string()
}

fn default_border_radius() -> u32 {
    16
}

impl Config {
    fn get_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("glint/config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::get_path();

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Self::default_config(),
        };

        match toml::from_str(&content) {
            Ok(config) => config,
            Err(e) => {
                // Now you get helpful error messages in the terminal!
                eprintln!("⚠️ Config Error at {:?}: {}", path, e);
                eprintln!("Using fallback defaults for missing or broken fields.");
                Self::default_config()
            }
        }
    }

    fn default_config() -> Self {
        Self {
            title: default_title(),
            tasks: default_tasks(),
            monitor_name: default_monitor_name(),
            anchor_bottom: default_true(),
            anchor_right: default_true(),
            margin_bottom: default_margin(),
            margin_right: default_margin(),
            background_color: default_bg(),
            text_color: default_text(),
            font_size: default_font_size(),
            border_width: default_border_width(),
            border_color: default_border_color(),
            border_radius: default_border_radius(),
        }
    }

    pub fn format_markdown(&self) -> String {
        let items: Vec<String> = self.tasks.iter().map(|t| format!("• {}", t)).collect();
        format!("<b>{}</b>\n\n{}", self.title, items.join("\n"))
    }
}
