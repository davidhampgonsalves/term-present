use std::fmt;

use utils;

pub mod text;
pub mod list;
pub mod bold;
pub mod slide;
pub mod paragraph;
pub mod code_block;

#[derive(Clone, Debug, PartialEq)]
pub enum ElementType { Text, List, Bold, Slide, Paragraph, CodeBlock, InlineCode }

#[derive(Clone, PartialEq)]
pub struct Fragment {
  pub element_type: ElementType,
  pub content: String,
  pub children: Option<Vec<Box<Fragment>>>,
}

impl Default for Fragment {
  fn default () -> Fragment {
    Fragment { element_type: ElementType::Text, content: "".to_string(), children: None }
  }
}

impl fmt::Debug for Fragment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}: '{}' {:?}", self.element_type, self.content, self.children)
    /* for c in self.children.unwrap() { c.fmt(f); }; */
    /* write!(f, "--") */
  }
}

pub fn is_block_element(fragment:&Fragment) -> bool {
  match fragment.element_type {
    ElementType::List => true,
    ElementType::CodeBlock => true,
    ElementType::Paragraph => true,
    ElementType::Slide => true,
    _ => false
  }
}

pub fn generate(fragment:&Fragment, indent:u16, max_width:u16) -> String {
  match fragment.element_type {
    ElementType::List => list::generate(fragment, indent, max_width),
    ElementType::Text => text::generate(fragment, indent, max_width),
    ElementType::Bold => bold::generate(fragment, indent, max_width),
    ElementType::Slide => slide::generate(fragment, indent, max_width),
    ElementType::Paragraph => paragraph::generate(fragment, indent, max_width),
    ElementType::CodeBlock => code_block::generate(fragment, indent, max_width),
    _ => "-NO ELEMENT HANDLER-".to_string(),
  }
}

pub fn generate_block(fragment:&Fragment, indent:u16, max_width:u16) -> String {
  fragment.children.as_ref().unwrap_or(&vec![]).iter().enumerate().map(|(i, item)| {
    let content = generate(item, indent, max_width);

    if is_block_element(item) {
      return content
    }
    return format!("{indent}-{content}\n", indent=utils::spaces(indent), content=content)
  }).collect()
}
