use gtk::{Image, gio, glib};
use zbus::blocking::{Connection, Proxy};

pub fn network_widget() -> Image {
    let icon = Image::new();
    icon.set_pixel_size(22);

    let icon_clone = icon.clone();

    let (sender, receiver) = async_channel::unbounded::<String>();

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = receiver.recv().await {
            icon_clone.set_icon_name(Some(&msg));
        }
    });

    gio::spawn_blocking(move || {
        let conn = Connection::system().unwrap();

        let proxy_nm_device = Proxy::new(
            &conn,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        )
        .unwrap();

        let changes = proxy_nm_device.receive_property_changed::<u32>("Connectivity");

        for signal in changes {
            let signal = signal.get().unwrap();

            let res = match signal {
                1 => "network-wireless-no-route-symbolic",  //Unknown
                2 => "network-wireless-disabled-symbolic",  //none
                3 => "network-wireless-connected-symbolic", //limited
                4 => "network-wireless-connected-symbolic", //full
                _ => "",                                    //ignore
            };

            //            println!("{signal}");

            sender
                .send_blocking(res.to_string())
                .expect("Channel closed.");
        }
    });

    icon
}
