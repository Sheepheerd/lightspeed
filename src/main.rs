use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::gdk::prelude::*;
use gtk::{gdk, FlowBox};
use gtk::{prelude::*, Label};

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
            .opacity(0.5)
            .build();

        let flowbox = FlowBox::builder()
            .max_children_per_line(26)
            .column_spacing(0)
            .row_spacing(0)
            .selection_mode(gtk::SelectionMode::None)
            .width_request(width)
            .height_request(height)
            .vexpand(false)
            .hexpand(false)
            .margin_bottom(0)
            .margin_top(0)
            .margin_end(0)
            .margin_start(0)
            .build();

        let label_map: Rc<RefCell<HashMap<String, Label>>> = Rc::new(RefCell::new(HashMap::new()));
        let key_buffer: Rc<RefCell<Vec<char>>> = Rc::new(RefCell::new(Vec::new()));

        for c2 in b'A'..=b'Z' {
            for c1 in b'A'..=b'Z' {
                let key = format!("{}{}", c1 as char, c2 as char);
                let label = Label::builder()
                    .label(&key)
                    .name(&key)
                    .width_request(0)
                    .height_request(0)
                    .vexpand_set(true)
                    .hexpand_set(true)
                    .hexpand(false)
                    .vexpand(false)
                    .halign(gtk::Align::Center)
                    .valign(gtk::Align::Center)
                    .margin_top(0)
                    .margin_bottom(0)
                    .margin_start(0)
                    .margin_end(0)
                    .build();

                flowbox.insert(&label, -1);
                label_map.borrow_mut().insert(key, label);
            }
        }

        let event_controller = EventControllerKey::new();
        let label_map_clone = Rc::clone(&label_map);
        let key_buffer_clone = Rc::clone(&key_buffer);
        let flowbox_clone = flowbox.clone();
        event_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, _state| {
            if let Some(c) = keyval.to_unicode() {
                let c = c.to_ascii_uppercase();
                if c.is_ascii_alphabetic() {
                    let mut buffer = key_buffer_clone.borrow_mut();
                    buffer.push(c);
                    if buffer.len() > 2 {
                        buffer.remove(0);
                    }
                    if buffer.len() == 2 {
                        let key = format!("{}{}", buffer[0], buffer[1]);
                        if let Some(label) = label_map_clone.borrow().get(&key) {
                            let new_grid = generate_grid(key.clone(), width, height);
                            flowbox_clone.remove(label);
                            // Insert the new grid in the same position
                            flowbox_clone.insert(&new_grid, get_index_of_letter_pair(&key));

                            // let mouse_manager = Mouse::new();
                            // mouse_manager.move_to();
                        }
                        buffer.pop();
                        buffer.pop();
                    }
                }
            }
            glib::Propagation::Proceed
        });

        window.set_child(Some(&flowbox));
        window.add_controller(event_controller);
        window.present();
    });

    app.run();
    // let mouse_manager = Mouse::new();
    // mouse_manager.move_to(48, 48);
}

fn generate_grid(key: String, width: i32, height: i32) -> FlowBox {
    let grid = FlowBox::builder()
        .max_children_per_line(3)
        .selection_mode(gtk::SelectionMode::None)
        .column_spacing(0)
        .row_spacing(0)
        // .width_request(30)
        // .height_request(30)
        .vexpand(true)
        .hexpand(true)
        .homogeneous(true)
        .margin_bottom(0)
        .margin_top(0)
        .margin_end(0)
        .margin_start(0)
        .build();

    // Generate a 3x3 grid based on the key
    let mut labels: Vec<String> = vec!["a", "b", "c", "d", "e", "f" /* "g", "h", "i" */]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    // Add the generated labels to the grid
    for label_key in labels {
        let label = Label::builder()
            .label(&label_key)
            .name(&label_key)
            // .width_request(1)
            // .height_request(1)
            .hexpand(false)
            .vexpand(false)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .margin_bottom(0)
            .margin_top(0)
            .margin_end(0)
            .margin_start(0)
            .build();
        grid.insert(&label, -1);
    }

    grid
}

fn get_index_of_letter_pair(pair: &str) -> i32 {
    if pair.len() != 2 {
        panic!("Input must be a two-character string");
    }

    let first_char = pair.chars().nth(0).unwrap();
    let second_char = pair.chars().nth(1).unwrap();

    let first_char_index = first_char as usize - 'A' as usize;
    let second_char_index = second_char as usize - 'A' as usize;
    ((second_char_index * 26) + first_char_index) as i32
}
