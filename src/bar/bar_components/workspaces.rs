use gtk::prelude::*;
use gtk::{Align::*, Box, Button, gio, glib};
use niri_ipc::{Event, Request, socket::Socket};

pub fn workspaces_widget() -> Box {
    let workspaces_box = Box::builder()
        .valign(Fill)
        .halign(Center)
        .spacing(12)
        .build();

    workspaces_box.set_css_classes(&["workspaces_widget"]);

    let mut workspaces: Vec<Button> = Vec::new();

    for i in 1..=6 {
        let button = Button::builder().valign(Center).build();

        button.set_css_classes(&["unfocused_workspace"]);

        workspaces.push(button.clone());

        workspaces_box.append(&button);
    }

    let (sender, receiver) = async_channel::unbounded::<usize>();

    glib::MainContext::default().spawn_local(async move {
        let mut current_workspace = 1;

        while let Ok(msg) = receiver.recv().await {
            workspaces[current_workspace].set_css_classes(&["unfocused_workspace"]);
            current_workspace = msg;
            workspaces[msg].set_css_classes(&["focused_workspace"]);
        }
    });

    gio::spawn_blocking(move || {
        let socket = Socket::connect().unwrap();

        match socket.send(Request::EventStream) {
            Ok((reply, mut event_fn)) => {
                while let Ok(event) = event_fn() {
                    if let Event::WorkspaceActivated { id, .. } = event {
                        sender
                            .send_blocking((id - 1) as usize)
                            .expect("Channel closed.");
                    }
                }
            }

            Err(e) => {
                println!("{e}");
            }
        };
    });

    workspaces_box
}
