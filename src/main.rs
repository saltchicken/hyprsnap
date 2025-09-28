use gtk4::gdk::{Display, Key, prelude::*};
use gtk4::EventControllerKey;
use gtk4::{Application, ApplicationWindow, CssProvider, Button, glib, Builder};
use gtk4::prelude::{WidgetExt, GtkWindowExt, ButtonExt};
use std::process::Command;

// 1. Embed the GtkBuilder XML file
const UI_FILE: &str = include_str!("./grid.ui");

// 2. Define the CSS
const CSS: &str = r#"
window.background {
    background-color: rgba(0, 0, 0, 0.5);
}

.custom-label {
    color: green;
}
"#;

fn main() {
    let app = Application::builder()
        .application_id("org.hyprland.transparentwidgetrust")
        .build();

    app.connect_activate(build_ui);
    
    app.run();
}

fn build_ui(app: &Application) {
    // 3. Load the UI from the embedded XML string
    let builder = Builder::from_string(UI_FILE);

    // Get the main window object
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
    
    // 5. Connect Signals
    let buttons_and_commands = vec![
        ("button_center", "dispatch setfloating 1; dispatch resizeactive exact 2560 1440; dispatch moveactive exact 1280 0"),
        ("button_left_full", "dispatch setfloating 1; dispatch resizeactive exact 1280 1440; dispatch moveactive exact 0 0"),
        ("button_right_full", "dispatch setfloating 1; dispatch resizeactive exact 1280 1440; dispatch moveactive exact 3840 0"),
        ("button_right_lower", "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 3840 720"),
        ("button_left_upper", "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 0 0"),
        ("button_right_upper", "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 3840 0"),
        ("button_left_lower", "dispatch setfloating 1; dispatch resizeactive exact 1280 720; dispatch moveactive exact 0 720"),
    ];

    for (button_id, command_str) in buttons_and_commands {
        let button: Button = builder
            .object(button_id)
            .unwrap_or_else(|| panic!("Could not get {} from builder.", button_id));
        
        button.connect_clicked(glib::clone!(@weak window, @strong command_str => move |_| {
            println!("Executing command: {}", command_str);
            match Command::new("hyprctl")
                .arg("--batch")
                .arg(&command_str)
                .spawn()
                     {
                    Ok(_) => println!("Command executed successfully"),
                    Err(e) => println!("Failed to execute command: {}", e),
                }
            window.close();
        }));
    }

    let key_controller = EventControllerKey::new();

    key_controller.connect_key_released(glib::clone!(@weak window => move |_, key, _, _| {
        if key == Key::Escape {
            window.close();
        }
    }));

    // window.set_focusable(true);
    // window.grab_focus();
    window.add_controller(key_controller);


    // 7. Show the Window
    window.present();
}

