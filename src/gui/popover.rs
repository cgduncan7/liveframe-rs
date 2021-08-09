use gtk::prelude::*;

pub enum PopoverState {
  Up,
  Down,
}

pub fn change_popover_state(builder: &gtk::Builder, state: PopoverState) {
  let popover: gtk::Popover = builder.get_object("popover-menu").unwrap();
  match state {
    PopoverState::Up => popover.popup(),
    PopoverState::Down => popover.popdown(),
  };
}