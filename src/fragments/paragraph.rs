use fragments;
use utils;

pub fn generate(fragment: &fragments::Fragment, indent:u16, max_width:u16) -> String {
  let content:String = fragment.children.as_ref().unwrap_or(&vec![]).iter().map(|f| fragments::generate(&f, indent, max_width)).collect();
  format!("{}{}\n", utils::spaces(indent), utils::wrap(content, indent, max_width))
}
