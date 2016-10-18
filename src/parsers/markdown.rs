use fragments;

pub use pulldown_cmark::Parser;
use pulldown_cmark::parse::{Event, Tag};
use pulldown_cmark::parse::Event::{Start, End, Text, Html, InlineHtml, SoftBreak, HardBreak, FootnoteReference};

fn parse(element_type: fragments::ElementType, parser: &mut Parser) -> Option<fragments::Fragment> {
  let mut content:String = "".to_string();
  let mut children: Vec<Box<fragments::Fragment>> = vec![];

  loop {
    let event = match parser.next() {
      Some(event) => { event },
      None => {
        if element_type == fragments::ElementType::Slide && !children.is_empty()  {
          return Some(fragments::Fragment { content: content, element_type: element_type, children: Some(children) });
        }
        return None
      }
    };

    println!("event{:?}", event);

    match event {
      Start(tag) => {
        if slide_end(&tag) {
          return Some(fragments::Fragment { content: content, element_type: element_type, children: Some(children) });
        }

        let child_element_type = to_presentation_tag(tag);
        let child_fragment = parse(child_element_type, parser);
        match child_fragment {
          Some(f) => { children.push(Box::new(f)); },
          None => { continue; }
        };
      },
      End(tag) => {
        return Some(fragments::Fragment { content: content, element_type: element_type, children: Some(children) })
      },
      Text(text) => {
        content = text.into_owned();
      },
      _ => panic!("here")
    }
  }

  panic!("end")
  /* None */
}

fn slide_end(tag: &Tag) -> bool {
  match *tag {
    Tag::Rule => { true },
    _ => { false }
  }
}

pub fn parse_slide(markdown: &str, index: i16) -> Option<fragments::Fragment> {
  let mut parser = Parser::new(markdown);
  let mut slides: Vec<fragments::Fragment> = vec![];
  let mut done = false;

  //TODO: skip slides * index
  parse(fragments::ElementType::Slide, &mut parser)
}

fn to_presentation_tag(tag: Tag) -> fragments::ElementType {
  match tag {
    Tag::Paragraph =>  { fragments::ElementType::Paragraph }
    Tag::List(_) => { fragments::ElementType::List },
    Tag::Item => { fragments::ElementType::Text },
    _ => {
      panic!("can't handle {:?}", tag)
    }
  }
}


#[cfg(test)]
mod test {
  use super::*;
  use fragments;

  #[test]
  fn parses_list() {
    let slide = parse_slide(&"* item 1\n* item 2\n".to_string(), 0).unwrap();

    let listItems = slide.children.unwrap()[0].clone().children.unwrap();

    assert_eq!(listItems.len(), 2);
    assert_eq!(listItems[0].content, "item 1");
    assert_eq!(listItems[1].content, "item 2");
  }
}
