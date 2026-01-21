use crate::config::Config;
use gtk4::prelude::*;
use gtk4::{
    glib, style_context_add_provider_for_display, Application, ApplicationWindow, CssProvider,
    Label,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub struct GlintApp {
    window: ApplicationWindow,
    label: Label,
    provider: CssProvider,
}

impl GlintApp {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder().application(app).build();

        window.init_layer_shell();
        window.set_layer(Layer::Background);
        window.add_css_class("glint-window");

        let label = Label::builder()
            .justify(gtk4::Justification::Left)
            .xalign(0.0)
            .wrap(true)
            .use_markup(true)
            .build();

        window.set_child(Some(&label));
        let provider = CssProvider::new();

        Self {
            window,
            label,
            provider,
        }
    }

    fn apply_ui(&self, config: &Config) {
        self.window.set_anchor(Edge::Bottom, config.anchor_bottom);
        self.window.set_anchor(Edge::Right, config.anchor_right);
        self.window.set_margin(Edge::Bottom, config.margin_bottom);
        self.window.set_margin(Edge::Right, config.margin_right);

        let css = format!(
            ".glint-window {{
                background-color: {bg}; 
                background-image: none; 
                border-radius: 16px; 
                min-width: 240px;
            }} 
            label {{ 
                color: {text}; 
                padding: 30px; 
                font-family: 'JetBrains Mono', 'sans-serif'; 
                font-size: {size}pt;
            }}",
            bg = config.background_color.trim(),
            text = config.text_color.trim(),
            size = config.font_size,
        );

        self.provider.load_from_data(&css);

        if let Some(display) = gtk4::gdk::Display::default() {
            style_context_add_provider_for_display(
                &display,
                &self.provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        }

        // fix windows not re-size issue and update content
        self.window.set_visible(false);
        self.label.set_markup(&config.format_markdown());
        self.window.set_size_request(-1, -1);
        self.window.set_default_size(0, 0);
        self.window.set_visible(true);
        self.window.queue_resize();
    }

    pub fn run(self) {
        let config = Config::load();
        self.apply_ui(&config);
        self.window.present();

        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            self.apply_ui(&Config::load());
            glib::ControlFlow::Continue
        });
    }
}
