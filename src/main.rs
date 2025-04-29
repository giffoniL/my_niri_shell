mod bar;
mod dashboard;
mod dock;

use gtk::prelude::*;
use gtk::{Application, CssProvider, gdk::Display, glib};

const APP_ID: &str = "my.niri.shell";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    //	let settings = Settings::for_display(&Display::default().expect("Could not connect to a display."));

    //	settings.set_gtk_theme_name(Some("empty"));

    let provider = CssProvider::new();
    provider.load_from_string(include_str!("css_file/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let dashboard = dashboard::Dashboard::new(app);

    let bar = bar::Bar::new(app, &dashboard);

    let dock = dock::Dock::new(app);

    bar.window.present();
    dock.dock_window.present();
}
