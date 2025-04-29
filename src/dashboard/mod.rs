use gtk::prelude::*;
use gtk::{Align::*, Application, Box, Button, Image, Revealer, RevealerTransitionType::*, Window};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::process::Command;

pub struct Dashboard {
    pub window: Window,
    pub revealer: Revealer,
}

impl Dashboard {
    pub fn new(app: &Application) -> Self {
        let window = Window::builder().application(app).build();

        //		window.set_default_size(0, 0);

        let revealer = Revealer::builder()
            .reveal_child(false)
            .transition_duration(500)
            .transition_type(SlideLeft)
            .valign(Center)
            .halign(End)
            .build();

        let main_box = Box::builder()
            .spacing(12)
            .orientation(gtk::Orientation::Vertical)
            .margin_end(16)
            .margin_top(16)
            .build();

        let shutdown_button = Button::new();
        //		shutdown_button.set_valign(Center);
        //		shutdown_button.set_halign(Center);

        let shutdown_icon = Image::from_icon_name("system-shutdown");
        shutdown_icon.set_pixel_size(80);
        shutdown_icon.set_valign(Center);
        shutdown_icon.set_halign(Center);

        shutdown_button.set_size_request(100, 100);

        shutdown_button.connect_clicked(|_| {
            Command::new("systemctl")
                .arg("poweroff")
                .status()
                .expect("Failed to execute command");
        });

        let reboot_button = Button::new();

        let reboot_icon = Image::from_icon_name("system-reboot");
        reboot_icon.set_pixel_size(80);
        reboot_icon.set_valign(Center);
        reboot_icon.set_halign(Center);

        reboot_button.set_size_request(100, 100);

        reboot_button.connect_clicked(|_| {
            Command::new("systemctl")
                .arg("reboot")
                .status()
                .expect("Failed to execute command");
        });

        let hibernate_button = Button::new();

        let hibernate_icon = Image::from_icon_name("system-hibernate");
        hibernate_icon.set_pixel_size(80);
        hibernate_icon.set_valign(Center);
        hibernate_icon.set_halign(Center);

        hibernate_button.set_size_request(100, 100);

        hibernate_button.connect_clicked(|_| {
            Command::new("systemctl")
                .arg("hibernate")
                .status()
                .expect("Failed to execute command");
        });

        // ================================================================================================
        // ================================================================================================

        main_box.set_css_classes(&["dashboard"]);

        shutdown_button.set_css_classes(&["power_buttons"]);
        reboot_button.set_css_classes(&["power_buttons"]);
        hibernate_button.set_css_classes(&["power_buttons"]);

        shutdown_icon.set_css_classes(&["power_icons"]);
        reboot_icon.set_css_classes(&["power_icons"]);
        hibernate_icon.set_css_classes(&["power_icons"]);

        // ================================================================================================
        // ================================================================================================

        window.set_child(Some(&revealer));

        revealer.set_child(Some(&main_box));
        main_box.append(&shutdown_button);
        shutdown_button.set_child(Some(&shutdown_icon));
        main_box.append(&reboot_button);
        reboot_button.set_child(Some(&reboot_icon));
        main_box.append(&hibernate_button);
        hibernate_button.set_child(Some(&hibernate_icon));

        // ================================================================================================
        // ================================================================================================

        window.init_layer_shell();

        window.set_layer(Layer::Overlay);

        let anchors = [
            (Edge::Left, false),
            (Edge::Right, true),
            (Edge::Top, true),
            (Edge::Bottom, false),
        ];

        for (anchor, state) in anchors {
            window.set_anchor(anchor, state);
        }

        Self { window, revealer }
    }
}
