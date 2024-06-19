use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, EntryBuffer, ListBox};

fn main() {
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK+3 Program")
            .build();
        let list_box = ListBox::builder().build();

        let button = Button::with_label("click me");
        let count = Arc::new(Mutex::new(0));
        let entry_buffer = EntryBuffer::new(Some("count 0"));
        list_box.add(&button);
        let entry = Entry::builder()
            .editable(false)
            .buffer(&entry_buffer)
            .build();
        button.connect_clicked(move |_| {
            let mut c = count.lock().unwrap();
            *c += 1;
            entry_buffer.set_text(&format!("count {}", &c));
        });

        list_box.add(&entry);
        window.add(&list_box);
        window.show_all();
    });
    app.run();
}