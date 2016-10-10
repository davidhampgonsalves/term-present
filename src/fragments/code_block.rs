use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use termion::{color, style};

use fragments;
use utils;

pub fn generate(fragment: &fragments::Fragment, indent:u16, max_width:u16) -> String {
  // TODO: pass this in?
  let ps = SyntaxSet::load_defaults_nonewlines();
  let ts = ThemeSet::load_defaults();

  let syntax = ps.find_syntax_by_extension("rs").unwrap();
  let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

  fragment.content.lines().map(|line| {
    // TODO: extend line to full length?
    let ranges: Vec<(Style, &str)> = h.highlight(line);
    let content = as_24_bit_terminal_escaped(&ranges[..], true);
    format!("{indent}{content}\n{reset}", indent=utils::spaces(indent), content=content, reset=style::Reset)
  }).collect()
}
