use gtk::prelude::*;

#[path = "./popover.rs"] mod popover;

pub const VIEW_STACK_WINDOW: &str = "view-stack_window";
pub const VIEW_STACK_PHOTO: &str = "view-stack_photo";

pub fn change_view_stack_child(builder: &gtk::Builder, name: &str) {
  let view_stack: gtk::Stack = builder.get_object("view-stack").unwrap();
  view_stack.set_visible_child_name(name);
}
