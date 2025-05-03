use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, EventControllerKey};
use mouce::{Mouse, MouseActions};

fn main() {
    let app = Application::builder().application_id("lightspeed").build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1920)
            .default_height(1080)
            .title("lightspeed")
            .build();

        let event_controller = EventControllerKey::new();

        event_controller.connect_key_pressed(|_ctrl, keyval, keycode, state| {
            println!("key value: {:?}", keyval);
            println!("keycode: {:?}", keycode);
            println!("modifiers: {:?}", state);
            false.into()
        });
        window.add_controller(event_controller);
        window.set_resizable(false);
        // window.maximize();
        window.set_decorated(false);

        window.present();
        window.set_opacity(0.2f64);
    });

    app.run();

    // let mouse_manager = Mouse::new();
    // let _ = mouse_manager.move_to(1000, 1000);
    //
    // thread::sleep(Duration::from_millis(2));
    // let _ = mouse_manager.click_button(&MouseButton::Left);
}
