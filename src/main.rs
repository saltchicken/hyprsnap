use gtk4::gdk::{Display, Key, prelude::*};
use gtk4::glib::clone;
use gtk4::{Application, ApplicationWindow, Button, Builder, CssProvider, EventControllerKey, glib};
use gtk4::prelude::{ButtonExt, GtkWindowExt, WidgetExt};
use std::collections::HashMap;
use std::process::Command;

// 1. Embed the GtkBuilder XML file
const UI_FILE: &str = include_str!("./grid.ui");

// 2. Define the CSS
const CSS: &str = r#"
window {
    background-color: rgba(0, 0, 0, 0.5);
}
"#;

// Struct to hold all info about a button
struct ButtonInfo {
    id: &'static str,
    label: &'static str,
    command: &'static str,
    key: Key,
}

fn main() {
    let app = Application::builder()
        .application_id("org.hyprland.transparentwidgetrust")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let builder = Builder::from_string(UI_FILE);

    let window: ApplicationWindow = builder
        .object("main_window")
        .expect("Could not get main_window from builder.");

    window.set_application(Some(app));

    // 4. Apply Window Settings and CSS
    window.set_decorated(false);

    let provider = CssProvider::new();
    provider.load_from_data(CSS);

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // 5. Define buttons with their new labels, commands, and key triggers
    // FIX 1: Changed Key::KEY_1 to Key::_1, etc.
    let buttons_data = vec![
        ButtonInfo { id: "button_left_full",  label: "1. Left Full",   command: "dispatch setfloating 1; dispatch resizeactive exact 1280 1440; dispatch moveactive exact 0 0", key: Key::_1 },
        ButtonInfo { id: "button_left_upper", label: "2. Left Upper",  command: "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 0 0", key: Key::_2 },
        ButtonInfo { id: "button_left_lower", label: "3. Left Lower",  command: "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 0 720", key: Key::_3 },
        ButtonInfo { id: "button_center",     label: "4. Center",      command: "dispatch setfloating 1; dispatch resizeactive exact 2560 1440; dispatch moveactive exact 1280 0", key: Key::_4 },
        ButtonInfo { id: "button_right_upper",label: "5. Right Upper", command: "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 3840 0", key: Key::_5 },
        ButtonInfo { id: "button_right_lower",label: "6. Right Lower", command: "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 3840 720", key: Key::_6 },
        ButtonInfo { id: "button_right_full", label: "7. Right Full",  command: "dispatch setfloating 1; dispatch resizeactive exact 1280 1440; dispatch moveactive exact 3840 0", key: Key::_7 },
    ];
    
    let mut key_to_button_map: HashMap<Key, Button> = HashMap::new();

    for info in buttons_data {
        let button: Button = builder
            .object(info.id)
            .unwrap_or_else(|| panic!("Could not get {} from builder.", info.id));
        
        // Set the new label with the number
        button.set_label(info.label);
        
        let command_str = info.command.to_string(); // Prepare command for closure
        button.connect_clicked(clone!(@weak window => move |_| {
            println!("Executing command: {}", command_str);
            if let Err(e) = Command::new("hyprctl")
                .arg("--batch")
                .arg(&command_str)
                .spawn() {
                println!("Failed to execute command: {}", e);
            }
            window.close();
        }));

        key_to_button_map.insert(info.key, button);
    }

    // 6. Set up the key controller
    let key_controller = EventControllerKey::new();
    key_controller.connect_key_released(clone!(@weak window, @strong key_to_button_map => move |_, key, _, _| {

        if key == Key::Escape {
            window.close();
            return;
        }

        if let Some(button) = key_to_button_map.get(&key) {
            button.emit_clicked();
        }
    }));

    window.add_controller(key_controller);

    // 7. Show the Window
    window.present();
}
