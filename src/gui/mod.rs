use gdk_pixbuf::{Pixbuf, Colorspace};
use glib::{Bytes, clone};
use gtk::prelude::*;
use url::Url;

fn setup_menu_buttons(builder: &gtk::Builder) {
  let view_stack: gtk::Stack = builder.get_object("view-stack").unwrap();
  let popover: gtk::Popover = builder.get_object("popover-menu").unwrap();

  let window_btn: gtk::Button = builder.get_object("menubutton-window").unwrap();
  window_btn.connect_clicked(clone!(@strong view_stack, @strong popover => move |_| {
      view_stack.set_visible_child_name("page0");
      popover.popdown();
  }));

  let photo_btn: gtk::Button = builder.get_object("menubutton-photo").unwrap();
  photo_btn.connect_clicked(clone!(@strong view_stack, @strong popover => move |_| {
      view_stack.set_visible_child_name("page1");
      popover.popdown();
  }));
}

fn fetch_image(builder: &gtk::Builder) {
  let photo_box: gtk::Box = builder.get_object("photo-box").unwrap();
  let url: Url = match Url::parse("http://home.collinduncan.com:54321/images/window.jpg") {
    Ok(url) => url,
    Err(_) => panic!("error parsing url for photo")
  };

  let bytes = reqwest::blocking::Client::new()
    .get(url)
    .send().unwrap()
    .bytes().unwrap();
    
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
