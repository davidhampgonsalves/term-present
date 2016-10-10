extern crate syntect;
extern crate termion;
extern crate pulldown_cmark;

mod fragments;
mod utils;
mod parsers;

/* pub use fragments::ElementType::{Text, List}; */

fn main() {
  let slide = fragments::Fragment {
    element_type: fragments::ElementType::List,
    children: Some(vec![
      Box::new(fragments::Fragment {
        element_type: fragments::ElementType::CodeBlock,
        content: "let arg = 1;\narg+1\n".to_string(),
        ..Default::default()
      }),
      Box::new(fragments::Fragment {
        element_type: fragments::ElementType::Bold,
        content: "list item 1".to_string(),
        ..Default::default()
      }),
      Box::new(fragments::Fragment {
        element_type: fragments::ElementType::List,
        children: Some(vec![
          Box::new(fragments::Fragment {
            content: "single nested list item 1".to_string(),
            ..Default::default()
          }),
          Box::new(fragments::Fragment {
            element_type: fragments::ElementType::List,
            children: Some(vec![
              Box::new(fragments::Fragment {
                content: "double nested list item 1".to_string(),
                ..Default::default()
              }),
              Box::new(fragments::Fragment {
                element_type: fragments::ElementType::Bold,
                content: "nested text".to_string(),
                ..Default::default()
              }),
            ]),
            ..Default::default()
          }),
        ]),
        ..Default::default()
      }),
      Box::new(fragments::Fragment {
        content: "list item 2".to_string(),
        ..Default::default()
      }),
      Box::new(fragments::Fragment {
        element_type: fragments::ElementType::Paragraph,
        children: Some(vec![
          Box::new(fragments::Fragment {
            content: "first, ".to_string(),
            ..Default::default()
          }),
          Box::new(fragments::Fragment {
            element_type: fragments::ElementType::Bold,
            content: "second, ".to_string(),
            ..Default::default()
          }),
          Box::new(fragments::Fragment {
            content: "third".to_string(),
            ..Default::default()
          }),
        ]),
        ..Default::default()
      }),
    ]),
    ..Default::default()
  };

  let str = fragments::generate(&slide, 10, 100);
  println!("---\n{}\n---", str);
}

