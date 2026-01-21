mod app;
mod config;

use app::GlintApp;
use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    let app = Application::builder()
        .application_id("byte4cat.dev.glint")
        .build();

    app.connect_activate(|app| {
        let glint = GlintApp::new(app);
        glint.run();
    });

    app.run();
}
