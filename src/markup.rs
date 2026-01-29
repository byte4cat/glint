use crate::config::Config;
use html2pango::markup_html;
use pulldown_cmark::{html, Options, Parser};

pub fn to_pango(content: &str, config: &Config) -> String {
    let mut lines = content.lines();

    let first_line = lines.next().unwrap_or("Glint");
    let title_text = first_line.trim_start_matches('#').trim();
    let remaining_body: String = lines.collect::<Vec<&str>>().join("\n");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&remaining_body, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("html_output: {}", html_output);

    let fortified_html = html_output
        .replace("<h2>", "<h2><b>")
        .replace("</h2>", "</b></h2>")
        .replace("<h3>", "<h3><b>")
        .replace("</h3>", "</b></h3>");

    println!("fortified_html: {}", fortified_html);

    let mut pango_body = markup_html(&format!("<body>{}</body>", fortified_html))
        .unwrap_or_else(|_| gtk4::glib::markup_escape_text(&remaining_body).to_string());

    let font_list = config.font_family.join(", ");
    pango_body = pango_body
        .replace(
            "<tt>",
            &format!(
                "<span font_family='{}' background='#313244' foreground='#f5e0dc' rise='-1000'> ",
                font_list
            ),
        )
        .replace("</tt>", " </span>")
        .replace("&quot;", "\"")
        .replace("&apos;", "'");

    println!("pango_body: {}", pango_body);

    format!(
        "<span font_weight='bold' size='{}pt' color='{}'>{}</span>\n\n{}",
        config.title_size,
        config.border_color,
        gtk4::glib::markup_escape_text(title_text),
        pango_body
    )
}
