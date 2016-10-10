use fragments;
use utils;

pub fn generate(fragment: &fragments::Fragment, indent:u16, max_width:u16) -> String {
  fragments::generate_block(&fragment, indent, max_width)
}
