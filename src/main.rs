use gtk::gdk;
use gtk::gdk::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, EventControllerKey};
use mouce::{Mouse, MouseActions};
// use gtk::gdk;

fn main() {
    let app = Application::builder().application_id("lightspeed").build();

    app.connect_activate(|app| {
        let mut width = 0;
        let mut height = 0;
        if let Some(display) = gdk::Display::default() {
            let monitors = display.monitors();
            let mut monitor_count = 0;
            for i in 0..monitors.n_items() {
                if let Some(monitor) = monitors
                    .item(i)
                    .and_then(|obj| obj.downcast::<gdk::Monitor>().ok())
                {
                    monitor_count += 1;
                    let geometry = monitor.geometry();
                    width = geometry.width();
                    height = geometry.height();
                }
            }
            if monitor_count > 1 {
                println!("\x1b[31mWAY TO MANY MONITORS MAN!\x1b[0m");
                app.quit();
                return;
            }
        }

        let window = ApplicationWindow::builder()
            .default_height(height)
            .default_width(width)
            .application(app)
            .title("lightspeed")
            .decorated(false)
            .resizable(false)
            .maximized(true)
            .opacity(0.2)
            .build();

        let event_controller = EventControllerKey::new();
        event_controller.connect_key_pressed(|_ctrl, keyval, keycode, state| {
            println!("key value: {:?}", keyval);
            println!("keycode: {:?}", keycode);
            println!("modifiers: {:?}", state);
            false.into()
        });

        window.add_controller(event_controller);
        window.present();
    });

    app.run();
}
