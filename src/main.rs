use mouce::common::MouseButton;
use mouce::{Mouse, MouseActions};
use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, EventControllerKey};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder().application(app).build();

        let event_controller = EventControllerKey::new();

        event_controller.connect_key_pressed(|_ctrl, keyval, keycode, state| {
            println!("key value: {:?}", keyval);
            println!("keycode: {:?}", keycode);
            println!("modifiers: {:?}", state);
            false.into()
        });
        window.add_controller(event_controller);
        window.set_resizable(false);
        window.present();
        window.set_opacity(0.2f64);
    });

    app.run();

    let mouse_manager = Mouse::new();
    let _ = mouse_manager.move_to(1000, 1000);

    thread::sleep(Duration::from_millis(2));
    let _ = mouse_manager.click_button(&MouseButton::Left);
}
