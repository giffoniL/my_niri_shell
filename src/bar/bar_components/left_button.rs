use gtk::Button;
use gtk::prelude::*;

pub fn left_button_widget() -> Button {
    let left_button = Button::builder().label("\u{eae7}").build();

    left_button.set_css_classes(&["left_button_widget"]);

    left_button
}
