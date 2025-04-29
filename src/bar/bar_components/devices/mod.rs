pub mod battery_device_label;
pub mod network;
pub mod sound_output_device_label;

use battery_device_label::*;
use network::*;
use sound_output_device_label::*;

use gtk::prelude::*;
use gtk::{Box, Button};

pub fn devices_widget() -> Button {
    let devices_box = Box::builder().spacing(12).build();

    let audio_icon = sound_output_device_icon();

    let battery_icon = battery_widget();

    let network_icon = network_widget();

    devices_box.append(&audio_icon);
    devices_box.append(&battery_icon);
    devices_box.append(&network_icon);

    let devices_button = Button::builder().child(&devices_box).build();

    devices_button.set_css_classes(&["devices_widget"]);

    devices_button
}
