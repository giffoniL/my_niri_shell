use gtk::{Image, gio, glib};
use zbus::blocking::{Connection, Proxy};

pub fn battery_widget() -> Image {
    let icon = Image::new();
    icon.set_pixel_size(24);

    let icon_clone = icon.clone();

    let (sender, receiver) = async_channel::unbounded::<String>();

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = receiver.recv().await {
            icon_clone.set_icon_name(Some(&msg));
        }
    });

    gio::spawn_blocking(move || {
        let conn = Connection::system().unwrap();

        let proxy_upower_device = Proxy::new(
            &conn,
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower/devices/battery_BAT0",
            "org.freedesktop.UPower.Device",
        )
        .unwrap();

        let percentage_changes = proxy_upower_device.receive_property_changed::<f64>("Percentage");

        for signal in percentage_changes {
            let signal = signal.get().unwrap() as u8;

            let battery_symbol = match signal {
                /*                90..=100 => "\u{e1a4}",
                80..=89 => "\u{ebd2}",
                60..=79 => "\u{ebd4}",
                40..=59 => "\u{ebe2}",
                20..=39 => "\u{ebdd}",
                6..=19 => "\u{ebe0}",
                0..=5 => "\u{ebd9}", */
                90..=100 => "battery-level-100-symbolic",
                80..=89 => "battery-level-90-symbolic",
                70..=79 => "battery-level-80-symbolic",
                60..=69 => "battery-level-70-symbolic",
                50..=59 => "battery-levle-60-symbolic",
                40..=49 => "battery-level-50-symbolic",
                30..=39 => "battery-level-40-symbolic",
                20..=29 => "battery-level-30-symbolic",
                10..=19 => "battery-level-20-symbolic",
                0..=9 => "battery-level-10-symbolic",
                _ => "What?",
            };

            sender
                .send_blocking(battery_symbol.to_string())
                .expect("Channel closed.");
        }
    });

    icon
}
