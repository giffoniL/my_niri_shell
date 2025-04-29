mod app_getter;
use app_getter::*;

use gtk::prelude::*;
use gtk::{
    Align::*, Application, Box, Button, EventControllerMotion, Image, Label, Popover, Revealer,
    RevealerTransitionType::*, Window, glib, glib::*,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::collections::BTreeMap;

pub struct Dock {
    pub dock_window: Window,
    pub apps_to_display: BTreeMap<String, (String, String, String)>,
}

impl Dock {
    pub fn new(app: &Application) -> Self {
        let dock_window = Window::builder().application(app).build();

        dock_window.set_size_request(0, 14);

        let motion_controller = EventControllerMotion::new();

        let dock_revealer = Revealer::builder()
            .transition_type(SlideUp)
            .transition_duration(500)
            .reveal_child(false)
            .halign(Center)
            .valign(End)
            .build();

        let main_box = app_box();

        let separator = gtk::Separator::new(gtk::Orientation::Vertical);
        separator.set_width_request(2);
        separator.set_margin_start(6);
        separator.set_margin_end(6);

        let config_button = Button::new();

        let config_icon = Image::from_icon_name("gnome-settings");
        config_icon.set_pixel_size(44);

        // ================================================================================================
        // ================================================================================================

        motion_controller.connect_enter(clone!(
            #[weak]
            dock_window,
            #[weak]
            dock_revealer,
            move |_, _, _| {
                dock_window.set_default_size(0, 0);
                dock_revealer.set_reveal_child(true);
            }
        ));

        motion_controller.connect_leave(clone!(
            #[weak]
            dock_window,
            #[weak]
            dock_revealer,
            move |_| {
                dock_revealer.set_reveal_child(false);
                dock_window.set_default_size(0, 14);
            }
        ));

        // ================================================================================================
        // ================================================================================================

        dock_revealer.set_css_classes(&["dock_box"]);

        config_icon.set_css_classes(&["dock_icons"]);

        separator.set_css_classes(&["dock_separator"]);

        // ================================================================================================
        // ================================================================================================

        dock_window.add_controller(motion_controller);

        dock_window.set_child(Some(&dock_revealer));

        dock_revealer.set_child(Some(&main_box));

        main_box.append(&separator);

        main_box.append(&config_button);

        config_button.set_child(Some(&config_icon));

        // ================================================================================================
        // ================================================================================================

        dock_window.init_layer_shell();

        dock_window.set_layer(Layer::Overlay);

        let anchors = [
            (Edge::Left, false),
            (Edge::Right, false),
            (Edge::Top, false),
            (Edge::Bottom, true),
        ];

        for (anchor, state) in anchors {
            dock_window.set_anchor(anchor, state);
        }

        let apps_to_display = apps_to_display();

        Self {
            dock_window,
            apps_to_display,
        }
    }
}
