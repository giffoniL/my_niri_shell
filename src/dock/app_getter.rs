use gtk::prelude::*;
use gtk::{Box, Button, Image};
use ini::Ini;
use std::collections::BTreeMap;
use std::process::Command;
use std::{fs, path::Path, path::PathBuf};

pub fn app_box() -> Box {
    let apps = apps_to_display();

    //	let mut buttons: Vec<Button> = Vec::new();

    let apps_box = Box::builder()
        .spacing(6)
        .orientation(gtk::Orientation::Horizontal)
        .build();

    for (_app, props) in apps {
        let app_button = Button::new();

        let app_icon = Image::from_icon_name(&props.1);
        app_icon.set_pixel_size(44);
        app_icon.set_css_classes(&["dock_icons"]);

        app_button.connect_clicked(move |_| {
            //			let to_exec = props.1.clone();
            Command::new("dex")
                .arg(&props.2)
                .spawn()
                .expect("Failed executing command.");
        });

        apps_box.append(&app_button);

        app_button.set_child(Some(&app_icon));
    }

    apps_box
}

pub fn apps_to_display() -> BTreeMap<String, (String, String, String)> {
    let home_folder = std::env::var_os("HOME")
        .map(PathBuf::from)
        .expect("Couldn't find home directory.");

    let data_file = home_folder
        .join(".local")
        .join("share")
        .join("niri_shell_data.json");

    let mut apps_to_display: BTreeMap<String, (String, String, String)> = BTreeMap::new();

    if data_file.exists() && data_file.is_file() {
        let file_readings = fs::read_to_string(data_file).unwrap();

        let apps_to_display: BTreeMap<String, (String, String, String)> =
            serde_json::from_str(&file_readings).unwrap();

        apps_to_display
    } else {
        let desktop_apps_folder = Path::new("/usr/share/applications/");

        for entry in fs::read_dir(desktop_apps_folder).unwrap() {
            let entry = entry.unwrap();

            let path = entry.path();

            if path.is_file() {
                let ini_reading = Ini::load_from_file(&path).unwrap();

                let file_name = path.to_str().unwrap();

                let name = ini_reading
                    .get_from(Some("Desktop Entry"), "Name")
                    .unwrap_or("None");

                let terminal_or_not = ini_reading
                    .get_from(Some("Desktop Entry"), "Terminal")
                    .unwrap_or("None");

                let type_ = ini_reading
                    .get_from(Some("Desktop Entry"), "Type")
                    .unwrap_or("None");
                let icon_name = ini_reading
                    .get_from(Some("Desktop Entry"), "Icon")
                    .unwrap_or("None");
                let to_exec = ini_reading
                    .get_from(Some("Desktop Entry"), "Exec")
                    .unwrap_or("None");
                let no_display = ini_reading
                    .get_from(Some("Desktop Entry"), "NoDisplay")
                    .unwrap_or("false");

                if (no_display == "false" || no_display == "None")
                    && type_ == "Application"
                    && terminal_or_not == "false"
                {
                    apps_to_display.insert(
                        name.to_string(),
                        (
                            to_exec.to_string(),
                            icon_name.to_string(),
                            file_name.to_string(),
                        ),
                    );
                }
            }
        }

        fs::create_dir_all(data_file.parent().unwrap()).unwrap();

        let json_string = serde_json::to_string_pretty(&apps_to_display).unwrap();

        let _ = fs::write(data_file, json_string);

        apps_to_display
    }
}
