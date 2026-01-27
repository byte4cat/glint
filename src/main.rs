mod app;
mod config;
mod markup;
mod style;

use app::GlintApp;
use gtk4::prelude::*;
use gtk4::Application;
use std::env;

#[cfg(not(target_os = "linux"))]
compile_error!("Glint is a Linux-only project leveraging Wayland Layer Shell.");

fn main() {
    if env::var("WAYLAND_DISPLAY").is_err() {
        eprintln!("Error: Glint requires a Wayland compositor (like Hyprland or Sway).");
        std::process::exit(1);
    }

    let app = Application::builder()
        .application_id("byte4cat.dev.glint")
        .build();

    app.connect_activate(|app| {
        let glint = GlintApp::new(app);
        glint.run();
    });

    app.run();
}
