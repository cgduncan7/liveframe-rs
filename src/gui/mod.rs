extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use glib::clone;

fn setup_menu_buttons(builder: &gtk::Builder) {
  let view_stack: gtk::Stack = builder.get_object("view-stack").unwrap();

  let window_btn: gtk::Button = builder.get_object("menubutton-window").unwrap();
  window_btn.connect_clicked(clone!(@strong view_stack => move |_| {
      view_stack.set_visible_child_name("page0");
  }));

  let photo_btn: gtk::Button = builder.get_object("menubutton-photo").unwrap();
  photo_btn.connect_clicked(clone!(@strong view_stack => move |_| {
      view_stack.set_visible_child_name("page1");
  }));
}

pub fn start_gui(app: &gtk::Application) {
  let glade_src = include_str!("../../assets/liveframe_layout.glade");
  let builder = gtk::Builder::from_string(glade_src);

  setup_menu_buttons(&builder);

  let window: gtk::ApplicationWindow = builder.get_object("main-window").unwrap();
  window.set_application(Some(app));
  window.show_all();
}
