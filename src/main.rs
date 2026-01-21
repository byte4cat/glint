use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, CssProvider, Label};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use serde::Deserialize;
use std::{fs, u32};

#[derive(Deserialize, Clone, Default)]
struct Config {
    title: String,
    tasks: Vec<String>,
    anchor_bottom: bool,
    anchor_right: bool,
    margin_bottom: i32,
    margin_right: i32,
    background_color: String,
    text_color: String,
    font_size: u32,
}

fn default_config() -> Config {
    Config {
        title: "✨ GLINT".to_string(),
        tasks: vec!["Create ~/.config/glint/config.toml".to_string()],
        anchor_bottom: true,
        anchor_right: true,
        margin_bottom: 40,
        margin_right: 40,
        background_color: "1e1e2e".to_string(),
        text_color: "#cdd6f4".to_string(),
        font_size: 14,
    }
}

fn get_config() -> Config {
    let config_path = format!(
        "{}/.config/glint/config.toml",
        std::env::var("HOME").unwrap()
    );
    fs::read_to_string(&config_path)
        .ok()
        .and_then(|content| toml::from_str::<Config>(&content).ok())
        .unwrap_or_else(default_config)
}

fn apply_style(config: &Config) {
    let provider = CssProvider::new();
    let css = format!(
        ".glint-window {{ 
            background-color: {bg}; 
            background-image: none; 
            border-radius: 16px; 
            min-width: 200px;
            box-shadow: none;
            border: none;
        }} 
        label {{ 
            color: {text}; 
            padding: 30px; 
            font-family: 'JetBrains Mono', 'sans-serif'; 
            font-size: {font_size}pt;
        }}",
        bg = config.background_color,
        text = config.text_color,
        font_size = config.font_size,
    );
    provider.load_from_data(&css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_USER,
        );
    }
}

fn apply_window_layout(window: &ApplicationWindow, config: &Config) {
    // 設定錨點
    window.set_anchor(Edge::Bottom, config.anchor_bottom);
    window.set_anchor(Edge::Top, !config.anchor_bottom);
    window.set_anchor(Edge::Right, config.anchor_right);
    window.set_anchor(Edge::Left, !config.anchor_right);

    // 設定邊距 (四個邊都更新以確保位置正確)
    window.set_margin(Edge::Bottom, config.margin_bottom);
    window.set_margin(Edge::Top, config.margin_bottom);
    window.set_margin(Edge::Right, config.margin_right);
    window.set_margin(Edge::Left, config.margin_right);
}

fn main() {
    let app = Application::builder()
        .application_id("byte4cat.dev.glint")
        .build();

    app.connect_activate(|app| {
        let config = get_config();
        let window = ApplicationWindow::builder().application(app).build();

        window.add_css_class("glint-window");

        window.init_layer_shell();
        window.set_layer(Layer::Background);

        let label = Label::builder()
            .justify(gtk4::Justification::Left)
            .xalign(0.0)
            .wrap(true)
            .build();

        // 初始設定
        apply_window_layout(&window, &config);
        apply_style(&config);
        label.set_text(&format!("{}\n\n{}", config.title, config.tasks.join("\n")));

        // 實時更新
        let label_clone = label.clone();
        let window_clone = window.clone();

        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            let new_config = get_config();
            apply_window_layout(&window_clone, &new_config);
            apply_style(&new_config);
            label_clone.set_text(&format!(
                "{}\n\n{}",
                new_config.title,
                new_config.tasks.join("\n")
            ));
            // update window size
            window_clone.set_default_size(0, 0);
            glib::ControlFlow::Continue
        });

        window.set_child(Some(&label));
        window.present();
    });

    app.run();
}
