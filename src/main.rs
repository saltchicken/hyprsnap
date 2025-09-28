use gtk4::gdk::{Display, Key, prelude::*};
use gtk4::EventControllerKey;
use gtk4::{Application, ApplicationWindow, CssProvider, Button, glib, Builder};
use gtk4::prelude::{WidgetExt, GtkWindowExt, ButtonExt};

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
    
    // 5. Connect Signals (Example: Center button)
    let center_button: Button = builder
        .object("button_1_1")
        .expect("Could not get button_1_1 from builder. Ensure it's defined in grid.ui.");

    // This now works because ButtonExt is imported
    center_button.connect_clicked(|_| {
        println!("Center button (1, 1) clicked!");
    });

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

