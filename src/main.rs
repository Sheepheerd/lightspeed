use gdk::EventKey;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn main() -> glib::ExitCode {
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

    app.run()
}
