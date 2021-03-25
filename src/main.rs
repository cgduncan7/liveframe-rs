extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.collinduncan.liveframe"),
        Default::default(),
    ).expect("GTK init failed...");

    application.connect_activate(|app| {
        let glade_src = include_str!("../assets/liveframe_layout.glade");
        let builder = gtk::Builder::from_string(glade_src);

        let window: gtk::ApplicationWindow = builder.get_object("main-window").unwrap();
        window.set_application(Some(app));
        window.show_all();
    });

    application.run(&[]);
}
