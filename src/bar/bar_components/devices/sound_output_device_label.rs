use gtk::{Image, gio, glib};
use pipewire::{context::Context, main_loop::MainLoop, registry::GlobalObject, types::ObjectType};

pub fn sound_output_device_icon() -> Image {
    let icon = Image::new();
    icon.set_pixel_size(22);

    let icon_clone = icon.clone();

    let (sender, receiver) = async_channel::unbounded::<String>();

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = receiver.recv().await {
            let msg_str = msg.as_str();
            match msg_str {
                "Headphones" => {
                    // label_clone.set_text("\u{f01f}");
                    icon_clone.set_icon_name(Some("audio-headphones-symbolic"));
                }
                "Speakers" => {
                    //label_clone.set_text("\u{e32d}");
                    icon_clone.set_icon_name(Some("audio-speakers-symbolic"));
                }
                _ => (),
            }
        }
    });

    gio::spawn_blocking(move || {
        let mainloop = MainLoop::new(None).unwrap();
        let context = Context::new(&mainloop).unwrap();
        let core = context.connect(None).unwrap();
        let registry = core.get_registry().unwrap();

        let _listener = registry
            .add_listener_local()
            .global(move |global| match global {
                GlobalObject {
                    type_: ObjectType::Node,
                    props: Some(props),
                    ..
                } => match props.get("node.nick") {
                    Some("Headphones") => {
                        sender
                            .send_blocking("Headphones".to_string())
                            .expect("Channel closed.");
                    }
                    Some("Speaker") => {
                        sender
                            .send_blocking("Speakers".to_string())
                            .expect("Channel closed.");
                    }
                    Some(other) => (),
                    None => (),
                },

                _ => (),
            })
            .register();

        mainloop.run();
    });

    icon
}
