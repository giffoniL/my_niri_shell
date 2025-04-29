pub mod bar_components;

use crate::dashboard::*;
use bar_components::{clock::*, devices::*, right_button::*, workspaces::*};

use gtk::prelude::*;
use gtk::{Align::*, Application, Box, CenterBox, Window};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub struct Bar {
    pub window: Window,
}

impl Bar {
    pub fn new(app: &Application, dashboard: &Dashboard) -> Self {
        let window = Window::builder()
            .application(app)
            .height_request(20)
            .build();

        let main_box = CenterBox::builder().valign(Fill).halign(Fill).build();
        main_box.set_css_classes(&["bar"]);

        let start_box = Box::builder().spacing(6).valign(Fill).halign(Start).build();

        //        let left_button = left_button_widget();

        let workspaces_widget = workspaces_widget();

        let middle_box = Box::builder()
            .spacing(6)
            .valign(Fill)
            .halign(Center)
            .build();

        let end_box = Box::builder().spacing(6).valign(Fill).halign(End).build();

        let devices_widget = devices_widget();

        let clock_widget = clock_widget();

        let right_button = right_button_widget(dashboard);

        // ================================================================================================
        // ================================================================================================
        window.set_child(Some(&main_box));

        main_box.set_start_widget(Some(&start_box));
        //      start_box.append(&left_button);
        start_box.append(&workspaces_widget);

        main_box.set_center_widget(Some(&middle_box));

        main_box.set_end_widget(Some(&end_box));
        end_box.append(&devices_widget);
        end_box.append(&clock_widget);
        end_box.append(&right_button);

        // ================================================================================================
        // ================================================================================================

        window.init_layer_shell();

        window.set_layer(Layer::Overlay);

        window.auto_exclusive_zone_enable();

        let anchors = [
            (Edge::Left, true),
            (Edge::Right, true),
            (Edge::Top, true),
            (Edge::Bottom, false),
        ];

        for (anchor, state) in anchors {
            window.set_anchor(anchor, state);
        }

        Self { window }
    }
}
