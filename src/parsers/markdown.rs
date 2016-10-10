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

/* pub fn parse(markdown: &str) -> Vec<fragments::Fragment> { */
/*   let mut parser = Parser::new(markdown); */
/*   let mut fragmentQueue: Vec<&fragments::Fragment> = vec![]; */
/*   let mut slides: Vec<fragments::Fragment> = vec![]; */

/*   while let Some(event) = parser.next() { */
/*     println!("{:?}", event); */
/*     if fragmentQueue.is_empty() { */
/*       fragmentQueue.push(&fragments::Fragment { element_type: fragments::ElementType::Slide, ..Default::default() }); */
/*     } */

/*     match event { */
/*       Start(tag) => { */
/*         let mut fragment = start_tag(tag, &fragmentQueue.last().unwrap()); */
/*         fragmentQueue.push(&fragment); */
/*       }, */
/*       End(tag) => { fragmentQueue.pop(); }, */
/*       Text(text) => { */
/*         fragmentQueue.last().unwrap().children.unwrap().push(Box::new(fragments::Fragment { */
/*           element_type: fragments::ElementType::Text, */
/*           content: text.to_string(), */
/*           ..Default::default() */
/*         })); */
/*       }, */
/*       _ => { */
/*         panic!("here") */
/*       } */
/*     } */
/*   } */
/*   vec![] */
/* } */

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
    println!("starting test");
    let slide = parse_slide(&"* item 1\n* item 2\n".to_string(), 0).unwrap();

    println!("parsed slide: {:?}", slide);
    let list = slide.children.unwrap().get(0);
    assert_eq!(list.unwrap().children.unwrap().len(), 2);
    /* assert_eq!(list[0].children.unwrap().len(), 2); */
  }
}
