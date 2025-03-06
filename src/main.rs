use mouce::common::MouseButton;
use mouce::{Mouse, MouseActions};
use std::thread;
use std::time::Duration;

use gdk::EventKey;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder().application(app).build();

        window.connect("key_press_event", false, |values| {
            let raw_event = &values[1].get::<gdk::Event>().unwrap();
            match raw_event.downcast_ref::<EventKey>() {
                Some(event) => {
                    println!("key value: {:?}", event.keyval());
                    println!("modifiers: {:?}", event.state());
                }
                None => {}
            }

            // Use a boolean type here, as returning a result for key press is necessary.
            let result = glib::value::Value::from_type(bool::static_type());
            Some(result)
        });
        window.set_resizable(false);
        window.present();
    });

    app.run();

    let mouse_manager = Mouse::new();
    let _ = mouse_manager.move_to(1000, 1000);

    thread::sleep(Duration::from_millis(2));
    let _ = mouse_manager.click_button(&MouseButton::Left);
}
