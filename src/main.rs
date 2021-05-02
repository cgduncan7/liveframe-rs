extern crate gtk;
extern crate gio;

use gio::prelude::*;

mod gui;

fn main() {
    let application = gtk::Application::new(
        Some("com.collinduncan.liveframe"),
        Default::default(),
    ).expect("GTK init failed...");

    application.connect_startup(move |app| {
        gui::start_gui(app);
    });

    application.connect_activate(|_| {});
    
    application.run(&[]);
}
