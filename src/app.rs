use notify::{RecursiveMode, Watcher};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::channel;

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
    current_config: RefCell<Config>,
}

impl GlintApp {
    pub fn new(app: &Application) -> Self {
        let config = Config::load()
            .expect("failed to load config.toml, please check ~/.config/glint/config.toml");
        let window = ApplicationWindow::builder().application(app).build();

        window.init_layer_shell();
        window.set_layer(Layer::Background);
        window.set_namespace("glint");
        window.add_css_class("glint-window");

        // on specific monitor
        let display = gtk4::gdk::Display::default().expect("Could not connect to a display.");
        let monitors = display.monitors();
        if let Some(ref target_name) = config.monitor_name {
            for i in 0..monitors.n_items() {
                if let Some(m) = monitors.item(i) {
                    let monitor = m
                        .downcast::<gtk4::gdk::Monitor>()
                        .expect("Failed to downcast monitor");
                    if monitor.connector().map(|s| s.to_string()) == Some(target_name.clone()) {
                        window.set_monitor(&monitor);
                        break;
                    }
                }
            }
        }

        if display.is_composited() {
            window.set_opacity(1.0);
        }

        // let label = Label::builder()
        //     .justify(gtk4::Justification::Left)
        //     .xalign(0.0)
        //     .wrap(false)
        //     .hexpand(false)
        //     .vexpand(false)
        //     .use_markup(true)
        //     .build();

        let label = Label::builder()
            .justify(gtk4::Justification::Left)
            .xalign(0.0)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .max_width_chars(50)
            .use_markup(true)
            .build();

        window.set_child(Some(&label));
        let provider = CssProvider::new();

        Self {
            window,
            label,
            provider,
            current_config: RefCell::new(config),
        }
    }

    fn apply_css(&self, config: &Config) {
        let ff_list = config.font_family.join(", ");
        let css = format!(
            ".glint-window {{
                background-color: {bg}; 
                background-image: none; 
                border-radius: {b_radius}px; 
                border: {b_width}px solid {b_color};
                min-width: 300px;
            }} 
            label {{ 
                color: {text}; 
                padding: 25px 30px; 
                font-family: {ff};
                font-size: {size}pt;
                margin: 0;
            }}",
            ff = ff_list,
            bg = config.background_color,
            b_width = config.border_width,
            b_color = config.border_color,
            b_radius = config.border_radius,
            text = config.text_color,
            size = config.font_size,
        );
        self.provider.load_from_data(&css);
    }

    // Only call this when we KNOW the config is different
    fn apply_ui(&self, config: &Config) {
        self.window.set_anchor(Edge::Bottom, config.anchor_bottom);
        self.window.set_anchor(Edge::Right, config.anchor_right);

        self.window.set_anchor(Edge::Top, false);
        self.window.set_anchor(Edge::Left, false);

        self.window.set_margin(Edge::Bottom, config.margin_bottom);
        self.window.set_margin(Edge::Right, config.margin_right);

        self.apply_css(config);

        if let Some(display) = gtk4::gdk::Display::default() {
            style_context_add_provider_for_display(
                &display,
                &self.provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        }

        // The "violent" update only happens when this function is called
        self.window.set_visible(false);
        self.label.set_markup(&config.format_markdown());
        self.window.set_size_request(-1, -1);
        let (_, natural_size) = self.label.preferred_size();
        self.window
            .set_default_size(natural_size.width() + 10, natural_size.height() + 10);
        self.window.queue_resize();
        self.window.set_visible(true);
    }

    pub fn run(self) {
        let app_rc = Rc::new(self);

        {
            let initial_config = app_rc.current_config.borrow().clone();
            app_rc.apply_ui(&initial_config);
        }

        app_rc.window.present();

        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
            if let Ok(event) = res {
                use notify::event::{EventKind, ModifyKind};

                match event.kind {
                    EventKind::Modify(ModifyKind::Data(_)) | EventKind::Modify(ModifyKind::Any) => {
                        let _ = tx.send(());
                    }
                    EventKind::Create(_) => {
                        let _ = tx.send(());
                    }
                    _ => {}
                }
            }
        })
        .expect("failed to create file change listener");

        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("glint");

        if config_dir.exists() {
            watcher.watch(&config_dir, RecursiveMode::NonRecursive).ok();
            println!("watching config directory: {:?}", config_dir);
        }

        let note_path_str = {
            let config = app_rc.current_config.borrow();
            shellexpand::tilde(&config.note_path).to_string()
        };
        let note_path = std::path::Path::new(&note_path_str);

        if let Some(note_dir) = note_path.parent() {
            if note_dir.exists() {
                let _ = watcher.watch(note_dir, RecursiveMode::NonRecursive);
                println!("watching note directory: {:?}", note_dir);
            }
        }

        Box::leak(Box::new(watcher));

        let app_clone = Rc::clone(&app_rc);
        glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
            let mut changed = false;

            while let Ok(_) = rx.try_recv() {
                changed = true;
            }

            if changed {
                println!("file change，hot reload...");
                if let Ok(new_config) = Config::load() {
                    app_clone.apply_ui(&new_config);
                    *app_clone.current_config.borrow_mut() = new_config;
                }
            }
            glib::ControlFlow::Continue
        });
    }
}
