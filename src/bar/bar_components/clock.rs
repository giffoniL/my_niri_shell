use glib::DateTime;
use gtk::prelude::*;
use gtk::{Label, gio, glib};

pub fn clock_widget() -> Label {
    let starting_time = DateTime::now_local()
        .unwrap()
        .format("%I:%M %p")
        .unwrap()
        .to_string();

    let label = Label::new(Some(&starting_time));

    label.set_css_classes(&["clock_widget"]);

    let label_clone = label.clone();

    let (sender, receiver) = async_channel::unbounded::<String>();

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = receiver.recv().await {
            label_clone.set_text(&msg);
        }
    });

    gio::spawn_blocking(move || {
        loop {
            let time = DateTime::now_local()
                .unwrap()
                .format("%a, %I:%M %p")
                .unwrap()
                .to_string();
            sender.send_blocking(time).expect("Channel closed.");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    label
}
