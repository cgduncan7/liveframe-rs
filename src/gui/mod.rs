use gdk_pixbuf::{Pixbuf, Colorspace};
use glib::{Bytes, clone};
use gtk::prelude::*;

#[path = "../network/mod.rs"] mod network;

mod popover;
mod viewstack;

fn setup_menu_buttons(builder: &gtk::Builder) {
  let window_btn: gtk::Button = builder.get_object("menubutton-window").unwrap();
  window_btn.connect_clicked(clone!(@strong builder => move |_| {
    viewstack::change_view_stack_child(&builder, viewstack::VIEW_STACK_WINDOW);
    popover::change_popover_state(&builder, popover::PopoverState::Down)
  }));

  let photo_btn: gtk::Button = builder.get_object("menubutton-photo").unwrap();
  photo_btn.connect_clicked(clone!(@strong builder => move |_| {
    viewstack::change_view_stack_child(&builder, viewstack::VIEW_STACK_PHOTO);
    popover::change_popover_state(&builder, popover::PopoverState::Down)
  }));
}

// TODO: spawn task to do this every minute
fn fetch_image(builder: &gtk::Builder) {
  let photo_box: gtk::Box = builder.get_object(viewstack::VIEW_STACK_PHOTO).unwrap();

  let result = network::perform_request(reqwest::Method::GET, "http://home.collinduncan.com:54321/images/window.jpg");

  let bytes = match result {
    Ok(resp) => resp.bytes().unwrap(),
    Err(_) => panic!("error with network request")
  };
    
  // TODO: clean this shit up
  let cursor = std::io::Cursor::new(bytes);
  let img = image::io::Reader::new(cursor).with_guessed_format().unwrap().decode().unwrap().into_rgb8();
  let resized_image = image::imageops::resize(&img, 780, 430, image::imageops::FilterType::Gaussian);
  let img_bytes = resized_image.into_vec();

  let bytes = Bytes::from_owned(img_bytes);

  let pb: Pixbuf = Pixbuf::from_bytes(
    &bytes,
    Colorspace::Rgb,
    false,
    8,
    780,
    430,
    Pixbuf::calculate_rowstride(Colorspace::Rgb, false, 8, 780, 430)
  );

  photo_box.add(&gtk::Image::from_pixbuf(Some(&pb)));
}

pub fn start_gui(app: &gtk::Application) {
  let glade_src = include_str!("../../assets/liveframe_layout.glade");
  let builder = gtk::Builder::from_string(glade_src);

  fetch_image(&builder);
  setup_menu_buttons(&builder);

  let window: gtk::ApplicationWindow = builder.get_object("main-window").unwrap();
  window.set_application(Some(app));
  window.show_all();
}
