use termion::{color, style};

use fragments;

pub fn generate(fragment: &fragments::Fragment, indent:u16, max_width:u16) -> String {
  format!("{bold}{content}{unbold}", bold=color::Fg(color::Blue), content=fragment.content, unbold=style::Reset)
}
