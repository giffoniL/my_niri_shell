use gtk::prelude::*;
use gtk::{Button, glib, glib::*};

use crate::dashboard::*;

pub fn right_button_widget(dashboard: &Dashboard) -> Button {
    let right_button = Button::builder().label("\u{e9ba}").build();

    right_button.set_css_classes(&["right_button_widget"]);

    let win = &dashboard.window;
    let rev = &dashboard.revealer;

    right_button.connect_clicked(clone!(
        #[weak]
        rev,
        #[weak]
        win,
        move |_| {
            let win_state = win.get_visible();
            let rev_state = rev.is_child_revealed();

            if win_state && rev_state {
                rev.set_reveal_child(false);
                timeout_add_local_once(
                    std::time::Duration::from_millis(500),
                    clone!(
                        #[weak]
                        win,
                        move || {
                            win.set_visible(false);
                        }
                    ),
                );

                rev.set_reveal_child(false);
            } else {
                win.set_visible(true);
                rev.set_reveal_child(true);
            };
        }
    ));

    right_button
}
