use crate::config::Config;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

pub fn to_pango(content: &str, config: &Config) -> String {
    let mut lines = content.lines();

    let first_line = lines.next().unwrap_or("Glint");
    let title_text = first_line.trim_start_matches('#').trim();
    let remaining_body: String = lines.collect::<Vec<&str>>().join("\n");

    let mut pango_output = String::with_capacity(content.len() * 2);

    pango_output.push_str(&format!(
        "<span font_weight='bold' size='{}pt' color='{}'>{}</span>\n\n",
        config.title_size,
        config.border_color,
        gtk4::glib::markup_escape_text(title_text)
    ));

    // 2. 處理 Body
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
                let font_list = config.font_family.join(", ");
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
