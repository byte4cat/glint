use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
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

    pub fn format_markdown(&self) -> String {
        let content = self.get_note_content();
        let mut lines = content.lines();

        // 修正 B: 抓取第一行當標題，剩下的當 body
        let first_line = lines.next().unwrap_or("Glint");
        let title_text = first_line.trim_start_matches('#').trim();
        let remaining_body: String = lines.collect::<Vec<&str>>().join("\n");

        let mut pango_output = format!(
            "<span font_weight='bold' size='{}pt' color='{}'>{}</span>\n\n",
            self.title_size,
            self.border_color,
            gtk4::glib::markup_escape_text(title_text)
        );

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&remaining_body, options);

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    let size = match level {
                        HeadingLevel::H1 => "x-large",
                        _ => "large",
                    };
                    pango_output.push_str(&format!("<span font_weight='bold' size='{}'>", size));
                }
                Event::End(TagEnd::Heading(_)) => pango_output.push_str("</span>\n"),
                Event::Start(Tag::Strong) => pango_output.push_str("<b>"),
                Event::End(TagEnd::Strong) => pango_output.push_str("</b>"),
                Event::TaskListMarker(checked) => {
                    let mark = if checked {
                        "<span color='#a6e3a1'>x</span>"
                    } else {
                        " "
                    };
                    pango_output.push_str(&format!("<b>[{}]</b> ", mark));
                }
                Event::Text(text) => pango_output.push_str(&gtk4::glib::markup_escape_text(&text)),
                Event::Code(code) => {
                    let font_list = self.font_family.join(", ");
                    pango_output.push_str(&format!(
                            "<span font_family='{}' background='#313244' foreground='#f5e0dc' rise='-1000'> {} </span>",
                            font_list,
                            gtk4::glib::markup_escape_text(&code)
                    ));
                }
                Event::SoftBreak | Event::HardBreak => pango_output.push_str("\n"),
                Event::Start(Tag::Item) => pango_output.push_str(" • "),
                Event::End(TagEnd::Item) => pango_output.push_str("\n"),
                _ => {}
            }
        }
        pango_output
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
