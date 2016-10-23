use fragments;

pub use pulldown_cmark::Parser;
use pulldown_cmark::parse::{Event, Tag};
use pulldown_cmark::parse::Event::{Start, End, Text, Html, InlineHtml, SoftBreak, HardBreak, FootnoteReference};


pub fn parse_slide(markdown: &str, index: i16) -> Option<fragments::Fragment> {
  let mut parser = Parser::new(markdown);
  let i = skip_to_slide(&mut parser, index);

  if i != index {
    parser = Parser::new(markdown);
    skip_to_slide(&mut parser, i);
  }

  // TODO: also return total number of slides (maybe reusing skip_to_slide
  parse(fragments::ElementType::Slide, &mut parser)
}

fn skip_to_slide(parser: &mut Parser, index: i16) -> i16 {
  if index == 0 { return 0 }
  let mut position: i16 = 0;

  loop {
    let event = match parser.next() {
      Some(event) => {
        match event {
          End(tag) => {
            if slide_end(&tag) {
              position += 1;
              if position == index {
                return position
              }
            }
          },
          _ => {}
        };
      },
      None => { return position }
    };
  }
}

fn parse(element_type: fragments::ElementType, parser: &mut Parser) -> Option<fragments::Fragment> {
  let mut content:String = "".to_string();
  let mut children: Vec<Box<fragments::Fragment>> = vec![];

  loop {
    let event = match parser.next() {
      Some(event) => { event },
      None => {
        if element_type != fragments::ElementType::Slide {
          panic!("Markdown data error, expected slide, got {:?}", element_type);
        }

        return Some(fragments::Fragment { content: content, element_type: fragments::ElementType::Slide, children: Some(children) });
      }
    };

    println!("event{:?}", event);

    match event {
      Start(tag) => {
        let child_element_type = to_presentation_tag(tag);
        let child_fragment = parse(child_element_type, parser);
        match child_fragment {
          Some(f) => { children.push(Box::new(f)); },
          None => { continue; }
        };
      },
      End(tag) => {
        /* if slide_end(&tag) { */
        /*   return Some(fragments::Fragment { content: content, element_type: element_type, children: Some(children) }); */
        /* } */

        return Some(fragments::Fragment { content: content, element_type: element_type, children: Some(children) })
      },
      Text(text) => {
        // if we aren't in a context type then create a text node otherwise set current node content
        if fragments::is_block_element(&element_type) {
          println!("adding content {}", text);
          children.push(Box::new(fragments::Fragment { content: text.into_owned(), ..Default::default()}));
        } else {
          content = text.into_owned();
        }
      },
      _ => panic!("here")
    }
  }
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

fn slide_end(tag: &Tag) -> bool {
  match *tag {
    Tag::Rule => { true },
    _ => { false }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use fragments;

  #[test]
  fn parses_list() {
    let slide = parse_slide(&"* item 1\n* item 2\n".to_string(), 0).unwrap();

    println!(">> {:?}", slide);
    let listItems = slide.children.unwrap()[0].clone().children.unwrap();

    assert_eq!(listItems.len(), 2);
    assert_eq!(listItems[0].content, "item 1");
    assert_eq!(listItems[1].content, "item 2");
  }

  #[test]
  fn parses_second_slide() {
    let slide = parse_slide(&"text\n\n---\nsecond slide content".to_string(), 1).unwrap();
    assert_eq!(slide.children.unwrap()[0].clone().children.unwrap()[0].content, "second slide content");
  }

  #[test]
  fn parses_last() {
    let slide = parse_slide(&"text\n\n---\nsecond slide content".to_string(), 4).unwrap();
    assert_eq!(slide.children.unwrap()[0].clone().children.unwrap()[0].content, "second slide content");
  }
}
